use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::{Context, anyhow};
use avina_wire::{
    budgeting::{UserBudget, UserBudgetModifyData},
    user::User,
};
use chrono::{Datelike, Utc};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::UserBudgetIdParam;
use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
    },
    database::{
        budgeting::{
            project_budget::select_maybe_project_budget_by_project_and_year_from_db,
            user_budget::select_user_budget_from_db,
        },
        user::user::select_user_from_db,
    },
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
    routes::{
        accounting::server_cost::get::ServerCostForProject,
        server_cost::get::calculate_server_cost_for_project,
    },
    utils::start_of_the_year,
};

#[tracing::instrument(name = "user_budget_modify")]
pub async fn user_budget_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<UserBudgetModifyData>,
    params: Path<UserBudgetIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    // TODO: do further validation
    if data.force {
        require_admin_user(&user)?;
    }
    if data.id != params.user_budget_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;

    let user_budget =
        select_user_budget_from_db(&mut transaction, data.id as u64).await?;
    let user_budget_user =
        select_user_from_db(&mut transaction, user_budget.user as u64).await?;
    require_master_user_or_return_not_found(&user, user_budget_user.project)?;

    let year = Utc::now().year();
    if user_budget.year < year as u32 && !data.force {
        return Err(OptionApiError::AuthorizationError(String::from(
            "Changing past budgets not allowed",
        )));
    }

    let end = Utc::now();
    let begin = start_of_the_year(user_budget.year);
    let ServerCostForProject::Detail(project_cost) =
        calculate_server_cost_for_project(
            &mut transaction,
            user.project as u64,
            begin,
            end,
            Some(true),
        )
        .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };

    let project_budget =
        select_maybe_project_budget_by_project_and_year_from_db(
            &mut transaction,
            user.project as u64,
            user_budget.year,
        )
        .await?;

    let amount = data.amount.unwrap() as f64;
    if {
        amount
            <= project_cost
                .users
                .get(&user_budget_user.name)
                .unwrap()
                .total
            || match project_budget {
                Some(project_budget) => {
                    project_budget.amount as f64 <= project_cost.total
                }
                None => false,
            }
    } && !data.force
    {
        return Err(OptionApiError::AuthorizationError(String::from(
            "Cost already exceeds desired budget amount",
        )));
    }

    let user_budget = update_user_budget_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user_budget))
}

#[tracing::instrument(
    name = "update_user_budget_in_db",
    skip(data, transaction)
)]
pub async fn update_user_budget_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &UserBudgetModifyData,
) -> Result<UserBudget, NotFoundOrUnexpectedApiError> {
    let row = select_user_budget_from_db(transaction, data.id as u64).await?;
    let amount = data.amount.unwrap_or(row.amount);
    let query = sqlx::query!(
        r#"
        UPDATE budgeting_userbudget
        SET amount = ?
        WHERE id = ?
        "#,
        amount,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let project = UserBudget {
        id: data.id,
        amount,
        user: row.user,
        username: row.username,
        year: row.year,
    };
    Ok(project)
}
