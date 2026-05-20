//! Implementation of the project-delete endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::ProjectIdParam;
use crate::{
    authorization::require_admin_user,
    database::user::project::delete_project_from_db, error::NormalApiError,
};

/// Delete the project with the given project ID.
///
/// This expects the project ID as URL path parameter. On success an HTTP 204 NO CONTENT status code is
/// returned.
///
/// Only admins can use this endpoint, otherwise an [NormalApiError::AuthorizationError] error is returned.
#[tracing::instrument(name = "project_delete")]
pub async fn project_delete(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_project_from_db(&mut transaction, params.project_id as u64).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}
