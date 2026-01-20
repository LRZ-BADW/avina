use std::collections::HashMap;

use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use anyhow::Context;
use avina_wire::user::{User, UserImport};
use chrono::{Datelike, Utc};
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
    error::NormalApiError,
    ldap::AvinaLdap,
    openstack::OpenStack,
    routes::{
        project::create::{NewProject, insert_project_into_db},
        user::user::create::{NewUser, insert_user_into_db},
    },
    startup::AvinaLdapConfig,
};

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
