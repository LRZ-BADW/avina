//! Queries for project budgets.

use anyhow::Context;
use avina_wire::budgeting::{ProjectBudget, ProjectBudgetCreateData};
use chrono::{Datelike, Utc};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

/// Select a project budget by the given ID from the database, or return [None].
#[tracing::instrument(
    name = "select_maybe_project_budget_from_db",
    skip(transaction)
)]
pub async fn select_maybe_project_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_budget_id: u64,
) -> Result<Option<ProjectBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, p.id as project, p.name as project_name, b.year, b.amount
        FROM budgeting_projectbudget as b, user_project as p
        WHERE
            b.project_id = p.id AND
            b.id = ?
        "#,
        project_budget_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => Some(
            ProjectBudget::from_row(&row)
                .context("Failed to parse project_budget row")?,
        ),
        None => None,
    })
}

/// Select a project budget with the given ID from the database, or a "not found" error.
///
/// This calls [select_maybe_project_budget_from_db] and then turns a [None] response into a
/// [NotFoundOrUnexpectedApiError::NotFoundError].
#[tracing::instrument(
    name = "select_project_budget_from_db",
    skip(transaction)
)]
pub async fn select_project_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_budget_id: u64,
) -> Result<ProjectBudget, NotFoundOrUnexpectedApiError> {
    select_maybe_project_budget_from_db(transaction, project_budget_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

/// Select a project budget by the given project ID and year from the database, or return [None].
///
/// There can only be one project budget per project and year.
#[tracing::instrument(
    name = "select_maybe_project_budget_by_project_and_year_from_db",
    skip(transaction)
)]
pub async fn select_maybe_project_budget_by_project_and_year_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    year: u32,
) -> Result<Option<ProjectBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, p.id as project, p.name as project_name, b.year, b.amount
        FROM budgeting_projectbudget as b, user_project as p
        WHERE
            b.project_id = p.id AND
            b.project_id = ? AND
            b.year = ?
        "#,
        project_id,
        year,
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => Some(
            ProjectBudget::from_row(&row)
                .context("Failed to parse project_budget row")?,
        ),
        None => None,
    })
}

/// Select a project budget by the given project ID and year from the database, or a "not found" error.
///
/// There can only be one project budget per project and year. This function calls
/// [select_maybe_project_budget_from_db] and then turns a [None] response into a
/// [NotFoundOrUnexpectedApiError::NotFoundError].
#[tracing::instrument(
    name = "select_project_budget_by_project_and_year_from_db",
    skip(transaction)
)]
pub async fn select_project_budget_by_project_and_year_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    year: u32,
) -> Result<ProjectBudget, NotFoundOrUnexpectedApiError> {
    select_maybe_project_budget_by_project_and_year_from_db(
        transaction,
        project_id,
        year,
    )
    .await?
    .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

/// Select a list of all project budgets from the database.
#[tracing::instrument(
    name = "select_all_project_budgets_from_db",
    skip(transaction)
)]
pub async fn select_all_project_budgets_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<ProjectBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, p.id as project, p.name as project_name, b.year, b.amount
        FROM budgeting_projectbudget as b, user_project as p
        WHERE b.project_id = p.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ProjectBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project budget")?;
    Ok(rows)
}

/// Select a list of project budgets by the given project ID from the database.
#[tracing::instrument(
    name = "select_project_budgets_by_project_from_db",
    skip(transaction)
)]
pub async fn select_project_budgets_by_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<ProjectBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, p.id as project, p.name as project_name, b.year, b.amount
        FROM budgeting_projectbudget as b, user_project as p
        WHERE
            b.project_id = p.id AND
            p.id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ProjectBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project budget")?;
    Ok(rows)
}

/// Select a list of project budgets by the given user ID from the database.
#[tracing::instrument(
    name = "select_project_budgets_by_user_from_db",
    skip(transaction)
)]
pub async fn select_project_budgets_by_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Vec<ProjectBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, p.id as project, p.name as project_name, b.year, b.amount
        FROM budgeting_projectbudget as b, user_project as p, user_user as u
        WHERE
            b.project_id = p.id AND
            p.id = u.project_id AND
            u.id = ?
        "#,
        user_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ProjectBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project budget")?;
    Ok(rows)
}

/// Select a list of project budgets by the given year from the database.
#[tracing::instrument(
    name = "select_project_budgets_by_year_from_db",
    skip(transaction)
)]
pub async fn select_project_budgets_by_year_from_db(
    transaction: &mut Transaction<'_, MySql>,
    year: u32,
) -> Result<Vec<ProjectBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, p.id as project, p.name as project_name, b.year, b.amount
        FROM budgeting_projectbudget as b, user_project as p
        WHERE
            b.project_id = p.id AND
            b.year = ?
        "#,
        year
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ProjectBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project budget")?;
    Ok(rows)
}

/// Simplified representation of data needed to create a new project budget.
pub struct NewProjectBudget {
    /// Project ID the budget belongs to.
    pub project_id: u64,
    /// Year the budget is for.
    pub year: u32,
    /// Amount the budget is set to (in EUR).
    pub amount: i64,
}

impl TryFrom<ProjectBudgetCreateData> for NewProjectBudget {
    type Error = String;

    /// Transform a [ProjectBudgetCreateData] into a [NewProjectBudget].
    ///
    /// More specifically this also replaces not inputted data by defaults, e.g.,
    /// 0. for the amount, and now in UTC for the start time.
    fn try_from(data: ProjectBudgetCreateData) -> Result<Self, Self::Error> {
        Ok(Self {
            project_id: data.project as u64,
            year: data.year.unwrap_or(Utc::now().year() as u32),
            amount: data.amount.unwrap_or(0),
        })
    }
}

/// Insert a new project budget based on the given [NewProjectBudget] into the database.
#[tracing::instrument(
    name = "insert_project_budget_into_db",
    skip(new_project_budget, transaction)
)]
pub async fn insert_project_budget_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_project_budget: &NewProjectBudget,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO budgeting_projectbudget (year, amount, project_id)
        VALUES (?, ?, ?)
        "#,
        new_project_budget.year,
        new_project_budget.amount,
        new_project_budget.project_id,
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new quota, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
