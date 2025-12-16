use anyhow::Context;
use avina_wire::resources::{
    FlavorGroup, FlavorGroupCreateData, FlavorGroupMinimal,
};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

#[tracing::instrument(
    name = "select_maybe_flavor_group_name_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_group_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<Option<String>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        name: String,
    }
    let query = sqlx::query!(
        r#"
        SELECT name
        FROM resources_flavorgroup
        WHERE id = ?
        "#,
        flavor_group_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse flavor group row")?
                .name,
        ),
        None => None,
    })
}

#[tracing::instrument(
    name = "select_flavor_group_name_from_db",
    skip(transaction)
)]
pub async fn select_flavor_group_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<String, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_group_name_from_db(transaction, flavor_group_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_maybe_flavor_group_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<Option<FlavorGroup>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            g.id as id,
            g.name as name,
            g.project_id as project,
            GROUP_CONCAT(f.id) as flavors
        FROM resources_flavorgroup as g
        LEFT JOIN resources_flavor as f
        ON g.id = f.group_id
        WHERE g.id = ?
        GROUP BY g.id
        "#,
        flavor_group_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            FlavorGroup::from_row(&row)
                .context("Failed to parse flavor group row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_group_from_db", skip(transaction))]
pub async fn select_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<FlavorGroup, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_group_from_db(transaction, flavor_group_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_maybe_lrz_flavor_group_from_db",
    skip(transaction)
)]
pub async fn select_maybe_lrz_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<Option<FlavorGroup>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            g.id as id,
            g.name as name,
            g.project_id as project,
            GROUP_CONCAT(f.id) as flavors
        FROM resources_flavorgroup as g
        LEFT JOIN resources_flavor as f
        ON g.id = f.group_id
        WHERE
            g.id = ? AND
            g.name like 'lrz.%'
        GROUP BY g.id
        "#,
        flavor_group_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            FlavorGroup::from_row(&row)
                .context("Failed to parse flavor group row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(
    name = "select_lrz_flavor_group_from_db",
    skip(transaction)
)]
pub async fn select_lrz_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<FlavorGroup, NotFoundOrUnexpectedApiError> {
    select_maybe_lrz_flavor_group_from_db(transaction, flavor_group_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_minimal_flavor_groups_by_project_id_from_db",
    skip(transaction)
)]
pub async fn select_minimal_flavor_groups_by_project_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<FlavorGroupMinimal>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            id,
            name
        FROM resources_flavorgroup
        WHERE project_id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorGroupMinimal::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor group")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_all_flavor_groups_from_db",
    skip(transaction)
)]
pub async fn select_all_flavor_groups_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<FlavorGroup>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            g.id as id,
            g.name as name,
            g.project_id as project,
            GROUP_CONCAT(f.id) as flavors
        FROM resources_flavorgroup as g
        LEFT JOIN resources_flavor as f
        ON g.id = f.group_id
        GROUP BY g.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorGroup::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor group")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_lrz_flavor_groups_from_db",
    skip(transaction)
)]
pub async fn select_lrz_flavor_groups_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<FlavorGroup>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            g.id as id,
            g.name as name,
            g.project_id as project,
            GROUP_CONCAT(f.id) as flavors
        FROM resources_flavorgroup as g
        LEFT JOIN resources_flavor as f
        ON g.id = f.group_id
        WHERE g.name LIKE 'lrz.%'
        GROUP BY g.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorGroup::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor group")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "insert_flavor_group_into_db",
    skip(new_flavor_group, transaction)
)]
pub async fn insert_flavor_group_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_flavor_group: &FlavorGroupCreateData,
    project_id: u64,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO resources_flavorgroup (name, project_id)
        VALUES (?, ?)
        "#,
        new_flavor_group.name,
        project_id,
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    // TODO: what about non-existing project_id?
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new flavor group, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
