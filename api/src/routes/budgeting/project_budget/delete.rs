use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::ProjectBudgetIdParam;
use crate::{
    authorization::require_admin_user,
    database::budgeting::project_budget::delete_project_budget_from_db,
    error::NormalApiError,
};

#[tracing::instrument(name = "project_budget_delete")]
pub async fn project_budget_delete(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<ProjectBudgetIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_project_budget_from_db(
        &mut transaction,
        params.project_budget_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}
