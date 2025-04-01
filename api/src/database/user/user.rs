use anyhow::Context;
use lrzcc_wire::user::{User, UserDetailed, UserMinimal};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};

#[tracing::instrument(
    name = "select_maybe_user_name_from_db",
    skip(transaction)
)]
pub async fn select_maybe_user_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Option<String>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        name: String,
    }
    let query = sqlx::query!(
        r#"
        SELECT name
        FROM user_user AS user
        WHERE user.id = ?
        "#,
        user_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse user row")?
                .name,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_user_name_from_db", skip(transaction))]
pub async fn select_user_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<String, NotFoundOrUnexpectedApiError> {
    select_maybe_user_name_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(name = "select_all_users_from_db", skip(transaction))]
pub async fn select_all_users_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<User>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            user.id AS id,
            user.name AS name,
            user.openstack_id AS openstack_id,
            user.role AS role,
            project.id as project,
            project.name AS project_name,
            user.is_staff AS is_staff,
            user.is_active AS is_active
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| User::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_users_by_project_from_db",
    skip(transaction)
)]
pub async fn select_users_by_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<User>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            user.id AS id,
            user.name AS name,
            user.openstack_id AS openstack_id,
            user.role AS role,
            project.id as project,
            project.name AS project_name,
            user.is_staff AS is_staff,
            user.is_active AS is_active
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id AND
            user.project_id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| User::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user")?;
    Ok(rows)
}

#[tracing::instrument(name = "select_users_by_id_db", skip(transaction))]
pub async fn select_users_by_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Vec<User>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            user.id AS id,
            user.name AS name,
            user.openstack_id AS openstack_id,
            user.role AS role,
            project.id as project,
            project.name AS project_name,
            user.is_staff AS is_staff,
            user.is_active AS is_active
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id AND
            user.id = ?
        "#,
        user_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| User::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_maybe_user_detail_from_db",
    skip(transaction)
)]
pub async fn select_maybe_user_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Option<UserDetailed>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            user.id AS id,
            user.name AS name,
            user.openstack_id AS openstack_id,
            user.role AS role,
            project.id as project__id,
            project.name AS project__name,
            project.user_class AS project__user_class,
            project.name AS project_name,
            user.is_staff AS is_staff,
            user.is_active AS is_active
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id AND
            user.id = ?
        "#,
        user_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            UserDetailed::from_row(&row).context("Failed to parse user row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_user_detail_from_db", skip(transaction))]
pub async fn select_user_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<UserDetailed, NotFoundOrUnexpectedApiError> {
    select_maybe_user_detail_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(name = "select_maybe_user_from_db", skip(transaction))]
pub async fn select_maybe_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Option<User>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            user.id AS id,
            user.name AS name,
            user.openstack_id AS openstack_id,
            user.role AS role,
            project.id as project,
            project.name AS project_name,
            user.is_staff AS is_staff,
            user.is_active AS is_active
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id AND
            user.id = ?
        "#,
        user_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => {
            Some(User::from_row(&row).context("Failed to parse user row")?)
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_user_from_db", skip(transaction))]
pub async fn select_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<User, NotFoundOrUnexpectedApiError> {
    select_maybe_user_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_minimal_users_by_project_id_from_db",
    skip(transaction)
)]
pub async fn select_minimal_users_by_project_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<UserMinimal>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name
        FROM user_user
        WHERE project_id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| UserMinimal::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to user")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_user_class_by_user_from_db",
    skip(transaction)
)]
pub async fn select_user_class_by_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Option<u32>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    struct Row {
        user_class: u32,
    }
    let query = sqlx::query!(
        r#"
        SELECT p.user_class as user_class
        FROM
            user_user AS u,
            user_project AS p
        WHERE
            u.project_id = p.id AND
            u.id = ?
        "#,
        user_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse user class row")?
                .user_class,
        ),
        None => None,
    })
}
