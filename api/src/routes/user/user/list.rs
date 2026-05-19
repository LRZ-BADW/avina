//! Implementation of the user-list endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, User, UserListParams};
use sqlx::MySqlPool;

use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
    },
    database::user::user::{
        select_all_users_from_db, select_users_by_id_from_db,
        select_users_by_project_from_db,
    },
    error::OptionApiError,
};

/// Get a list of users.
///
/// On success an HTTP 200 OK with the requested list in the response data is returned.
///
/// By default this only returns the current user in a list. To retrieve more, the
/// following filters may be used:
///
///   - `all`: returns all users in the system, can only be called by admin users, otherwise
///     an [OptionApiError::AuthorizationError] is returned.
///   - `project`: returns all users in the project with the given ID, can only be called by master
///     users of that project or admins, or otherwise returns an [OptionApiError::NotFoundError] instead
///     of an authorization error to prevent leaking existing but invisible project IDs to users.
///
/// Note, that given both filters, `all` takes precedence.
#[tracing::instrument(name = "user_list")]
pub async fn user_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<UserListParams>,
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let users = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_users_from_db(&mut transaction).await?
    } else if let Some(project_id) = params.project {
        require_master_user_or_return_not_found(&user, project_id)?;
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
