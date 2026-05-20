//! Implementation of the project-modify endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::{ProjectModifyData, User};
use sqlx::MySqlPool;

use super::ProjectIdParam;
use crate::{
    authorization::require_admin_user,
    database::user::project::update_project_in_db, error::OptionApiError,
};

/// Modify the project based on the given [ProjectModifyData].
///
/// This expects the project ID as path parameter, and the data to update in the request data.
/// On success a HTTP 200 OK with the updated project in the response data is returned.
///
/// Only admins can call this endpoint, otherwise an [OptionApiError::AuthorizationError] is returned.
/// If the ID given via URL parameters does not match the ID in the request data,
/// an [OptionApiError::ValidationError] is returned.
#[tracing::instrument(name = "project_modify")]
pub async fn project_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectModifyData>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.project_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project = update_project_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}
