use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    budgeting::{BudgetOverTree, BudgetOverTreeParams},
    user::User,
};
use chrono::Utc;
use sqlx::MySqlPool;

use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
        require_user_or_project_master_or_not_found,
    },
    database::user::user::select_user_from_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "budget_over_tree")]
pub async fn budget_over_tree(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Query<BudgetOverTreeParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let end = params.end.unwrap_or(Utc::now().fixed_offset());
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let over: BudgetOverTree = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        todo!();
    } else if let Some(project_id) = params.project {
        require_master_user_or_return_not_found(&user, project_id)?;
        todo!();
    } else if let Some(user_id) = params.user {
        let user_queried =
            select_user_from_db(&mut transaction, user_id as u64).await?;
        require_user_or_project_master_or_not_found(
            &user,
            user_id,
            user_queried.project,
        )?;
        todo!();
    } else {
        todo!();
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(over))
}
