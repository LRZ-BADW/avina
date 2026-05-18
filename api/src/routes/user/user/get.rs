//! Implementation of the user-get endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, User};
use sqlx::MySqlPool;

use super::UserIdParam;
use crate::{
    authorization::require_user_or_project_master_or_not_found,
    database::user::user::select_user_detail_from_db, error::OptionApiError,
};

/// Get the user with the given user ID.
#[tracing::instrument(name = "user_get")]
pub async fn user_get(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Path<UserIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let user_queried =
        select_user_detail_from_db(&mut transaction, params.user_id as u64)
            .await?;
    require_user_or_project_master_or_not_found(
        &user,
        user_queried.id,
        user_queried.project.id,
    )?;

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user_queried))
}
