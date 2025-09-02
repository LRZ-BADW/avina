use std::collections::HashMap;

use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use anyhow::Context;
use avina_wire::user::{User, UserClass, UserImport};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::user::{
        project::select_all_projects_from_db, user::select_all_users_from_db,
    },
    error::NormalApiError,
    openstack::OpenStack,
    routes::{
        project::create::{NewProject, insert_project_into_db},
        user::user::create::{NewUser, insert_user_into_db},
    },
};

#[tracing::instrument(name = "user_import", skip(openstack))]
pub async fn user_import(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    openstack: Data<OpenStack>,
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

    let mut new_user_count = 0;
    let mut new_project_count = 0;

    let mut domain_name_by_id = HashMap::new();
    for os_domain in os_domains {
        domain_name_by_id.insert(os_domain.id.clone(), os_domain.name.clone());
        if !project_names.contains(&os_domain.name) {
            let project_name = os_domain.name;

            let new_project = NewProject {
                name: project_name,
                openstack_id: os_domain.id,
                // TODO: get userclass from ldap
                user_class: UserClass::NA,
            };
            insert_project_into_db(&mut transaction, &new_project).await?;
            // TODO: create project budget

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

        let new_user = NewUser {
            name: os_project.name.clone(),
            openstack_id: os_project.id.clone(),
            project_id: project.id,
            // TODO: get role from ldap
            role: 1,
            is_staff: false,
            is_active: false,
        };
        insert_user_into_db(&mut transaction, &new_user).await?;
        // TODO: create user budget

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
