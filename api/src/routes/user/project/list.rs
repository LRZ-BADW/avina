//! Implementation of the project-list endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, ProjectListParams, User};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::user::project::{
        select_all_projects_from_db, select_projects_by_id_from_db,
        select_projects_by_userclass_from_db,
    },
    error::NormalApiError,
};

/// Get a list of projects.
///
/// On success an HTTP 200 OK with the requested list in the response data is returned.
///
/// By default this only returns the project of the calling user in a list. To retrieve more, admin
/// users may use the following filters:
///
///   - `all`: returns all projects in the system.
///   - `user_class`: returns all projects of the given user class.
///
/// Note, that given both filters, `all` takes precedence. If users other than admins use any of
/// them, an [OptionApiError::AuthorizationError] is returned.
#[tracing::instrument(name = "project_list")]
pub async fn project_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ProjectListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let projects = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_projects_from_db(&mut transaction).await?
    } else if let Some(userclass) = params.userclass {
        require_admin_user(&user)?;
        select_projects_by_userclass_from_db(&mut transaction, userclass)
            .await?
    } else {
        select_projects_by_id_from_db(&mut transaction, project.id as u64)
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(projects))
}
