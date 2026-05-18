//! Implementation of the user-list endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, User, UserListParams};
use sqlx::MySqlPool;

use crate::{
    authorization::{require_admin_user, require_master_user},
    database::user::user::{
        select_all_users_from_db, select_users_by_id_from_db,
        select_users_by_project_from_db,
    },
    error::NormalApiError,
};

/// Get a list of users.
///
/// By default this only returns the current user in a list. To retrieve more, the
/// following filters may be used:
///
///   * `all`: returns all users in the system, can only be called by admin users.
///   * `project`: returns all users in the project with the given ID, can only be called by master
///     users of that project or admins.
///
/// Note, that given both filters, `all` takes precedence.
#[tracing::instrument(name = "user_list")]
pub async fn user_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<UserListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let users = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_users_from_db(&mut transaction).await?
    } else if let Some(project_id) = params.project {
        require_master_user(&user, project_id)?;
        select_users_by_project_from_db(&mut transaction, project_id as u64)
            .await?
    } else {
        select_users_by_id_from_db(&mut transaction, user.id as u64).await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(users))
}
