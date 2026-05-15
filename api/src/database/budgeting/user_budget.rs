//! Queries for project budgets.

use anyhow::Context;
use avina_wire::budgeting::{UserBudget, UserBudgetCreateData};
use chrono::{Datelike, Utc};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

/// Select a user budget by the given ID from the database, or return [None].
#[tracing::instrument(
    name = "select_maybe_user_budget_from_db",
    skip(transaction)
)]
pub async fn select_maybe_user_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_budget_id: u64,
) -> Result<Option<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            b.id = ?
        "#,
        user_budget_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => Some(
            UserBudget::from_row(&row)
                .context("Failed to parse user_budget row")?,
        ),
        None => None,
    })
}

/// Select a user budget with the given ID from the database, or a "not found" error.
///
/// This calls [select_maybe_user_budget_from_db] and then turns a [None] response into a
/// [NotFoundOrUnexpectedApiError::NotFoundError].
#[tracing::instrument(name = "select_user_budget_from_db", skip(transaction))]
pub async fn select_user_budget_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_budget_id: u64,
) -> Result<UserBudget, NotFoundOrUnexpectedApiError> {
    select_maybe_user_budget_from_db(transaction, user_budget_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

/// Select a user budget by the given user ID and year from the database, or return [None].
///
/// There can only be one user budget per user and year.
#[tracing::instrument(
    name = "select_maybe_user_budget_by_user_and_year_from_db",
    skip(transaction)
)]
pub async fn select_maybe_user_budget_by_user_and_year_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    year: u32,
) -> Result<Option<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            u.id = ? AND
            b.year = ?
        "#,
        user_id,
        year
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => Some(
            UserBudget::from_row(&row)
                .context("Failed to parse user_budget row")?,
        ),
        None => None,
    })
}

/// Select a user budget by the given user ID and year from the database, or a "not found" error.
///
/// There can only be one user budget per user and year. This function calls
/// [select_maybe_user_budget_from_db] and then turns a [None] response into a
/// [NotFoundOrUnexpectedApiError::NotFoundError].
#[tracing::instrument(
    name = "select_user_budget_by_user_and_year_from_db",
    skip(transaction)
)]
pub async fn select_user_budget_by_user_and_year_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    year: u32,
) -> Result<UserBudget, NotFoundOrUnexpectedApiError> {
    select_maybe_user_budget_by_user_and_year_from_db(
        transaction,
        user_id,
        year,
    )
    .await?
    .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

/// Select a list of user budgets by the given project ID and year.
#[tracing::instrument(
    name = "select_user_budgets_by_project_and_year_from_db",
    skip(transaction)
)]
pub async fn select_user_budgets_by_project_and_year_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    year: u32,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            u.project_id = ? AND
            b.year = ?
        "#,
        project_id,
        year
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user budget")?;
    Ok(rows)
}

/// Select a list of all user budgets from the database.
#[tracing::instrument(
    name = "select_all_user_budgets_from_db",
    skip(transaction)
)]
pub async fn select_all_user_budgets_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE b.user_id = u.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user budget")?;
    Ok(rows)
}

/// Select a list of user budgets by the given project ID from the database.
#[tracing::instrument(
    name = "select_user_budgets_by_project_from_db",
    skip(transaction)
)]
pub async fn select_user_budgets_by_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            u.project_id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user budget")?;
    Ok(rows)
}

/// Select a list of user budgets by the given user ID from the database.
#[tracing::instrument(
    name = "select_user_budgets_by_user_from_db",
    skip(transaction)
)]
pub async fn select_user_budgets_by_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            u.id = ?
        "#,
        user_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user budget")?;
    Ok(rows)
}

/// Select a list of user budgets by the given year from the database.
#[tracing::instrument(
    name = "select_user_budgets_by_year_from_db",
    skip(transaction)
)]
pub async fn select_user_budgets_by_year_from_db(
    transaction: &mut Transaction<'_, MySql>,
    year: u32,
) -> Result<Vec<UserBudget>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, u.id as user, u.name as username, b.year, b.amount
        FROM budgeting_userbudget as b, user_user as u
        WHERE
            b.user_id = u.id AND
            b.year = ?
        "#,
        year
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserBudget::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user budget")?;
    Ok(rows)
}

/// Simplified representation of data needed to create a new project budget.
pub struct NewUserBudget {
    /// User ID the budget belongs to.
    pub user_id: u64,
    /// Year the budget is for.
    pub year: u32,
    /// Amount the budget is set to (in EUR).
    pub amount: i64,
}

impl TryFrom<UserBudgetCreateData> for NewUserBudget {
    type Error = String;

    /// Transform a [UserBudgetCreateData] into a [NewUserBudget].
    ///
    /// More specifically this also replaces not inputted data by defaults, e.g.,
    /// 0. for the amount, and now in UTC for the start time.
    fn try_from(data: UserBudgetCreateData) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: data.user as u64,
            year: data.year.unwrap_or(Utc::now().year() as u32),
            amount: data.amount.unwrap_or(0),
        })
    }
}

/// Insert a new user budget based on the given [NewUserBudget] into the database.
#[tracing::instrument(
    name = "insert_user_budget_into_db",
    skip(new_user_budget, transaction)
)]
pub async fn insert_user_budget_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_user_budget: &NewUserBudget,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO budgeting_userbudget (year, amount, user_id)
        VALUES (?, ?, ?)
        "#,
        new_user_budget.year,
        new_user_budget.amount,
        new_user_budget.user_id,
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

/// Synchronize next year's user budgets to the current one's.
///
/// This sets all user budgets for next year to the amount the respective user's budget has this
/// year.
#[tracing::instrument(name = "sync_user_budgets_in_db", skip(transaction))]
pub async fn sync_user_budgets_in_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<u64, MinimalApiError> {
    let year = Utc::now().year();
    let query = sqlx::query!(
        r#"
        UPDATE
            budgeting_userbudget AS c,
            budgeting_userbudget AS n
        SET n.amount = c.amount
        WHERE c.user_id = n.user_id
          AND c.year = ?
          AND n.year = ?
          AND c.amount != n.amount
        "#,
        year,
        year + 1
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    Ok(result.rows_affected())
}
