use anyhow::Context;
use avina_wire::user::{Project, ProjectMinimal, UserClass};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{NotFoundOrUnexpectedApiError, UnexpectedOnlyError};

#[tracing::instrument(name = "select_maybe_project_from_db", skip(transaction))]
pub async fn select_maybe_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Option<Project>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project AS project
        WHERE
            project.id = ?
        "#,
        project_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Project::from_row(&row).context("Failed to parse project row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_project_from_db", skip(transaction))]
pub async fn select_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Project, NotFoundOrUnexpectedApiError> {
    select_maybe_project_from_db(transaction, project_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_maybe_project_minimal_from_db",
    skip(transaction)
)]
pub async fn select_maybe_project_minimal_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Option<ProjectMinimal>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id as project__id,
            name as project__name,
            user_class as project__user_class
        FROM user_project AS project
        WHERE project.id = ?
        "#,
        project_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            ProjectMinimal::from_row(&row)
                .context("Failed to parse project row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(
    name = "select_project_minimal_from_db",
    skip(transaction)
)]
pub async fn select_project_minimal_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<ProjectMinimal, NotFoundOrUnexpectedApiError> {
    select_maybe_project_minimal_from_db(transaction, project_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_maybe_project_name_from_db",
    skip(transaction)
)]
pub async fn select_maybe_project_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Option<String>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        name: String,
    }
    let query = sqlx::query!(
        r#"
        SELECT name
        FROM user_project AS project
        WHERE
            project.id = ?
        "#,
        project_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse project row")?
                .name,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_project_name_from_db", skip(transaction))]
pub async fn select_project_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<String, NotFoundOrUnexpectedApiError> {
    select_maybe_project_name_from_db(transaction, project_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(name = "select_all_projects_from_db", skip(transaction))]
pub async fn select_all_projects_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<Project>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Project::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_projects_by_userclass_from_db",
    skip(transaction)
)]
pub async fn select_projects_by_userclass_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_class: UserClass,
) -> Result<Vec<Project>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        where user_class = ?
        "#,
        user_class as u32
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Project::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}

#[tracing::instrument(name = "select_projects_by_id_db", skip(transaction))]
pub async fn select_projects_by_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<Project>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name,
            openstack_id,
            user_class
        FROM user_project
        WHERE id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Project::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to project")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_user_class_by_project_from_db",
    skip(transaction)
)]
pub async fn select_user_class_by_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Option<UserClass>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    struct Row {
        user_class: u32,
    }
    let query = sqlx::query!(
        r#"
        SELECT user_class
        FROM user_project
        WHERE id = ?
        "#,
        project_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse user class row")?
                .user_class
                .try_into()
                .context("Failed to parse user class")?,
        ),
        None => None,
    })
}
