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
