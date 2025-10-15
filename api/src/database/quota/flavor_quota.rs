use anyhow::Context;
use avina_wire::quota::{FlavorQuota, FlavorQuotaCreateData};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

#[tracing::instrument(
    name = "select_maybe_flavor_quota_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_quota_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_quota_id: u64,
) -> Result<Option<FlavorQuota>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            q.id as id,
            u.id as user,
            u.name as username,
            q.quota as quota,
            g.id as flavor_group,
            g.name as flavor_group_name
        FROM
            quota_flavorquota as f,
            quota_quota as q,
            resources_flavorgroup as g,
            user_user as u
        WHERE
            f.quota_ptr_id = q.id AND
            f.flavor_group_id = g.id AND
            q.user_id = u.id AND
            q.id = ?
        "#,
        flavor_quota_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            FlavorQuota::from_row(&row)
                .context("Failed to parse flavor quota row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_quota_from_db", skip(transaction))]
pub async fn select_flavor_quota_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_quota_id: u64,
) -> Result<FlavorQuota, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_quota_from_db(transaction, flavor_quota_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_all_flavor_quotas_from_db",
    skip(transaction)
)]
pub async fn select_all_flavor_quotas_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<FlavorQuota>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            q.id as id,
            u.id as user,
            u.name as username,
            q.quota as quota,
            g.id as flavor_group,
            g.name as flavor_group_name
        FROM
            quota_flavorquota as f,
            quota_quota as q,
            resources_flavorgroup as g,
            user_user as u
        WHERE
            f.quota_ptr_id = q.id AND
            f.flavor_group_id = g.id AND
            q.user_id = u.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorQuota::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor quota")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_flavor_quotas_by_flavor_group_from_db",
    skip(transaction)
)]
pub async fn select_flavor_quotas_by_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<Vec<FlavorQuota>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            q.id as id,
            u.id as user,
            u.name as username,
            q.quota as quota,
            g.id as flavor_group,
            g.name as flavor_group_name
        FROM
            quota_flavorquota as f,
            quota_quota as q,
            resources_flavorgroup as g,
            user_user as u
        WHERE
            f.quota_ptr_id = q.id AND
            f.flavor_group_id = g.id AND
            q.user_id = u.id AND
            g.id = ?
        "#,
        flavor_group_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorQuota::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor quota")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_flavor_quotas_by_user_from_db",
    skip(transaction)
)]
pub async fn select_flavor_quotas_by_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Vec<FlavorQuota>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            q.id as id,
            u.id as user,
            u.name as username,
            q.quota as quota,
            g.id as flavor_group,
            g.name as flavor_group_name
        FROM
            quota_flavorquota as f,
            quota_quota as q,
            resources_flavorgroup as g,
            user_user as u
        WHERE
            f.quota_ptr_id = q.id AND
            f.flavor_group_id = g.id AND
            q.user_id = u.id AND
            u.id = ?
        "#,
        user_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorQuota::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor quota")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_maybe_flavor_quota_by_user_and_group_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_quota_by_user_and_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    flavor_group_id: u64,
) -> Result<Option<FlavorQuota>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            q.id as id,
            u.id as user,
            u.name as username,
            q.quota as quota,
            g.id as flavor_group,
            g.name as flavor_group_name
        FROM
            quota_flavorquota as f,
            quota_quota as q,
            resources_flavorgroup as g,
            user_user as u
        WHERE
            f.quota_ptr_id = q.id AND
            f.flavor_group_id = g.id AND
            q.user_id = u.id AND
            u.id = ? AND
            g.id = ?
        "#,
        user_id,
        flavor_group_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            FlavorQuota::from_row(&row)
                .context("Failed to parse flavor quota row")?,
        ),
        None => None,
    })
}

#[tracing::instrument(
    name = "select_flavor_quota_by_user_and_group_from_db",
    skip(transaction)
)]
pub async fn select_flavor_quota_by_user_and_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    flavor_group_id: u64,
) -> Result<FlavorQuota, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_quota_by_user_and_group_from_db(
        transaction,
        user_id,
        flavor_group_id,
    )
    .await?
    .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "insert_flavor_quota_into_db",
    skip(new_flavor_quota, transaction)
)]
pub async fn insert_flavor_quota_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_flavor_quota: &FlavorQuotaCreateData,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query1 = sqlx::query!(
        r#"
        INSERT IGNORE INTO quota_quota (quota, user_id)
        VALUES (?, ?)
        "#,
        new_flavor_quota.quota,
        new_flavor_quota.user
    );
    let result1 = transaction
        .execute(query1)
        .await
        .context("Failed to execute insert query")?;
    if result1.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new quota, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result1.last_insert_id();
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query2 = sqlx::query!(
        r#"
        INSERT IGNORE INTO quota_flavorquota (quota_ptr_id, flavor_group_id)
        VALUES (?, ?)
        "#,
        id,
        new_flavor_quota.flavor_group
    );
    let result2 = transaction
        .execute(query2)
        .await
        .context("Failed to execute insert query")?;
    if result2.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new flavor quota, a conflicting entry exists"
                .to_string(),
        ));
    }
    Ok(id)
}
