use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::{Context, anyhow};
use avina_wire::{
    budgeting::{ProjectBudget, ProjectBudgetModifyData},
    user::User,
};
use chrono::{Datelike, Utc};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::ProjectBudgetIdParam;
use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
    },
    database::budgeting::project_budget::select_project_budget_from_db,
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
    routes::{
        accounting::server_cost::get::ServerCostForProject,
        server_cost::get::calculate_server_cost_for_project,
    },
    utils::start_of_the_year,
};

#[tracing::instrument(name = "project_budget_modify")]
pub async fn project_budget_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectBudgetModifyData>,
    params: Path<ProjectBudgetIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    // TODO: do further validation
    if data.force {
        require_admin_user(&user)?;
    }
    if data.id != params.project_budget_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project_budget = select_project_budget_from_db(
        &mut transaction,
        params.project_budget_id as u64,
    )
    .await?;
    require_master_user_or_return_not_found(&user, project_budget.project)?;

    let year = Utc::now().year();
    if project_budget.year < year as u32 && !data.force {
        return Err(OptionApiError::AuthorizationError(String::from(
            "Changing past budgets not allowed",
        )));
    }

    let ServerCostForProject::Normal(cost) = calculate_server_cost_for_project(
        &mut transaction,
        project_budget.project as u64,
        start_of_the_year(year as u32),
        Utc::now(),
        None,
    )
    .await?
    else {
        return Err(anyhow!("Unexpected ServerCostForProject variant.").into());
    };

    if data.amount.unwrap() <= cost.total as u32 && !data.force {
        return Err(OptionApiError::AuthorizationError(String::from(
            "Cost already exceeds desired budget amount",
        )));
    }

    update_project_budget_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project_budget))
}

#[tracing::instrument(
    name = "update_project_budget_in_db",
    skip(data, transaction)
)]
pub async fn update_project_budget_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &ProjectBudgetModifyData,
) -> Result<ProjectBudget, NotFoundOrUnexpectedApiError> {
    let row =
        select_project_budget_from_db(transaction, data.id as u64).await?;
    let amount = data.amount.unwrap_or(row.amount);
    let query = sqlx::query!(
        r#"
        UPDATE budgeting_projectbudget
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
    let project = ProjectBudget {
        id: data.id,
        amount,
        project: row.project,
        project_name: row.project_name,
        year: row.year,
    };
    Ok(project)
}
