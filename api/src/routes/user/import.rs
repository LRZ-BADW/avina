use std::collections::HashMap;

use actix_web::{
    HttpResponse,
    http::StatusCode,
    web::{Data, ReqData},
};
use anyhow::{Context, anyhow};
use avina_wire::user::{User, UserClass, UserImport};
use chrono::{DateTime, Datelike, Utc};
use reqwest::Client;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::{
        budgeting::{
            project_budget::{NewProjectBudget, insert_project_budget_into_db},
            user_budget::{NewUserBudget, insert_user_budget_into_db},
        },
        user::{
            project::select_all_projects_from_db,
            user::select_all_users_from_db,
        },
    },
    error::{NormalApiError, UnexpectedOnlyError},
    openstack::OpenStack,
    routes::{
        project::create::{NewProject, insert_project_into_db},
        user::user::create::{NewUser, insert_user_into_db},
    },
    startup::AvinaLdapConfig,
};

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapUser {
    name: String,
    project: Option<String>,
    master: bool,
    function: bool,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapProject {
    name: String,
    class: u32,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapData {
    users: HashMap<String, AvinaLdapUser>,
    projects: HashMap<String, AvinaLdapProject>,
    timestamp: DateTime<Utc>,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapResponse(Option<AvinaLdapData>);

struct AvinaLdap {
    data: Option<AvinaLdapData>,
}

impl AvinaLdap {
    #[tracing::instrument(name = "call_avina_ldap")]
    async fn new(
        config: &AvinaLdapConfig,
    ) -> Result<Self, UnexpectedOnlyError> {
        let (url, token, default) = match &config {
            AvinaLdapConfig::Enabled(url, token, default) => {
                (url, token, default)
            }
            AvinaLdapConfig::Disabled(default) => {
                if !default {
                    return Err(anyhow!(
                        "avina-ldap disabled but using defaults also not configured."
                    ).into());
                }
                return Ok(Self { data: None });
            }
        };
        let response = Client::new()
            .get(url)
            .header("Authorization", token)
            .send()
            .await
            .context("Call to avina-ldap failed.")?;
        if response.status().as_u16() != StatusCode::OK {
            return Err(
                anyhow!("avina-ldap returned non-OK status code.").into()
            );
        }
        let data: AvinaLdapResponse = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read avina-ldap response text")?
                .as_str(),
        )
        .context("Could not parse avina-ldap response")?;
        if data.0.is_none() && !default {
            return Err(anyhow!(
                    "avina-ldap returned nothing but using defaults also not configured."
                ).into());
        }
        // TODO: we could also check if the data is outdated.
        Ok(Self { data: data.0 })
    }

    fn get_userclass(&self, project_name: &str) -> UserClass {
        // TODO: are these defaults the best way to handle this
        if let Some(data) = &self.data
            && let Some(project) = data.projects.get(project_name)
        {
            return UserClass::try_from(project.class).unwrap_or(UserClass::NA);
        }
        UserClass::NA
    }

    fn get_role(&self, username: &str) -> u32 {
        if let Some(data) = &self.data
            && let Some(user) = data.users.get(username)
            && user.master
            && !user.function
        {
            2
        } else {
            1
        }
    }
}

#[tracing::instrument(name = "user_import", skip(openstack))]
pub async fn user_import(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    openstack: Data<OpenStack>,
    avina_ldap_config: Data<AvinaLdapConfig>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;

    let os_domains = openstack.get_domains().await?;
    let os_projects = openstack.get_projects().await?;

    let users = select_all_users_from_db(&mut transaction).await?;
    let mut projects = select_all_projects_from_db(&mut transaction).await?;
    let usernames: Vec<String> = users.iter().map(|u| u.name.clone()).collect();
    let project_names: Vec<String> =
        projects.iter().map(|u| u.name.clone()).collect();
    let ldap_data = AvinaLdap::new(&avina_ldap_config).await?;

    let mut new_user_count = 0;
    let mut new_project_count = 0;

    let year = Utc::now().year() as u32;

    let mut domain_name_by_id = HashMap::new();
    for os_domain in os_domains {
        domain_name_by_id.insert(os_domain.id.clone(), os_domain.name.clone());
        if !project_names.contains(&os_domain.name) {
            let project_name = os_domain.name;

            let new_project = NewProject {
                name: project_name.clone(),
                openstack_id: os_domain.id,
                user_class: ldap_data.get_userclass(&project_name),
            };
            let project_id =
                insert_project_into_db(&mut transaction, &new_project).await?;
            insert_project_budget_into_db(
                &mut transaction,
                &NewProjectBudget {
                    project_id,
                    year,
                    amount: 0,
                },
            )
            .await?;

            new_project_count += 1;
        }
    }

    if new_project_count > 0 {
        projects = select_all_projects_from_db(&mut transaction).await?;
    }

    let project_by_name = projects
        .into_iter()
        .map(|p| (p.name.clone(), p))
        .collect::<HashMap<_, _>>();

    for os_project in os_projects
        .iter()
        .filter(|p| p.name.len() <= 16 && !usernames.contains(&p.name))
    {
        let Some(domain_name) = domain_name_by_id.get(&os_project.domain_id)
        else {
            continue;
        };
        let Some(project) = project_by_name.get(domain_name) else {
            continue;
        };

        let username = &os_project.name;
        let new_user = NewUser {
            name: username.clone(),
            openstack_id: os_project.id.clone(),
            project_id: project.id,
            role: ldap_data.get_role(username),
            is_staff: false,
            is_active: true,
        };
        let user_id = insert_user_into_db(&mut transaction, &new_user).await?;
        insert_user_budget_into_db(
            &mut transaction,
            &NewUserBudget {
                user_id,
                year,
                amount: 0,
            },
        )
        .await?;

        new_user_count += 1;
    }

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(UserImport {
            new_project_count,
            new_user_count,
        }))
}
