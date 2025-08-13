use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    budgeting::{BudgetOverTree, BudgetOverTreeParams, BudgetOverTreeUser},
    user::User,
};
use chrono::{DateTime, Utc};
use sqlx::{MySql, MySqlPool, Transaction};

use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
        require_user_or_project_master_or_not_found,
    },
    database::user::user::select_user_from_db,
    error::{OptionApiError, UnexpectedOnlyError},
};

async fn budget_over_tree_for_user(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    end: DateTime<Utc>,
) -> Result<BudgetOverTree, UnexpectedOnlyError> {
    todo!()
}

async fn budget_over_tree_for_project(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    end: DateTime<Utc>,
) -> Result<BudgetOverTree, UnexpectedOnlyError> {
    todo!()
}

async fn budget_over_tree_for_all(
    transaction: &mut Transaction<'_, MySql>,
    end: DateTime<Utc>,
) -> Result<BudgetOverTree, UnexpectedOnlyError> {
    todo!()
}

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
    let over = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        budget_over_tree_for_all(&mut transaction, end.into()).await?
    } else if let Some(project_id) = params.project {
        require_master_user_or_return_not_found(&user, project_id)?;
        budget_over_tree_for_project(
            &mut transaction,
            project_id as u64,
            end.into(),
        )
        .await?
    } else if let Some(user_id) = params.user {
        let user_queried =
            select_user_from_db(&mut transaction, user_id as u64).await?;
        require_user_or_project_master_or_not_found(
            &user,
            user_id,
            user_queried.project,
        )?;
        budget_over_tree_for_user(&mut transaction, user_id as u64, end.into())
            .await?
    } else {
        budget_over_tree_for_user(&mut transaction, user.id as u64, end.into())
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(over))
}
