use anyhow::Context;
use avina_wire::resources::{
    Flavor, FlavorCreateData, FlavorDetailed, FlavorGroupMinimal, FlavorMinimal,
};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

#[tracing::instrument(
    name = "select_maybe_flavor_name_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Option<String>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        name: String,
    }
    let query = sqlx::query!(
        r#"
        SELECT name
        FROM resources_flavor AS flavor
        WHERE flavor.id = ?
        "#,
        flavor_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse flavor row")?
                .name,
        ),
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_name_from_db", skip(transaction))]
pub async fn select_flavor_name_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<String, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_name_from_db(transaction, flavor_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(name = "select_maybe_flavor_from_db", skip(transaction))]
pub async fn select_maybe_flavor_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Option<Flavor>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            f.id,
            f.name,
            f.openstack_id,
            f.weight,
            f.group_id,
            g.name as group_name
        FROM resources_flavor as f
        LEFT JOIN resources_flavorgroup as g
        ON f.group_id = g.id
        WHERE f.id = ?
        "#,
        flavor_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => {
            Some(Flavor::from_row(&row).context("Failed to parse flavor row")?)
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_from_db", skip(transaction))]
pub async fn select_flavor_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Flavor, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_from_db(transaction, flavor_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_maybe_lrz_flavor_from_db",
    skip(transaction)
)]
pub async fn select_maybe_lrz_flavor_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Option<Flavor>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            f.id,
            f.name,
            f.openstack_id,
            f.weight,
            f.group_id,
            g.name as group_name
        FROM resources_flavor as f
        LEFT JOIN resources_flavorgroup as g
        ON f.group_id = g.id
        WHERE
            f.id = ? AND
            g.name like 'lrz.%'
        "#,
        flavor_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    // TODO: isn't there a nicer way to write this?
    Ok(match row {
        Some(row) => {
            Some(Flavor::from_row(&row).context("Failed to parse flavor row")?)
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_lrz_flavor_from_db", skip(transaction))]
pub async fn select_lrz_flavor_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Flavor, NotFoundOrUnexpectedApiError> {
    select_maybe_lrz_flavor_from_db(transaction, flavor_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_minimal_flavors_by_group_from_db",
    skip(transaction)
)]
pub async fn select_minimal_flavors_by_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    group_id: u64,
) -> Result<Vec<FlavorMinimal>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT f.id, f.name
        FROM resources_flavor as f
        WHERE f.group_id = ?
        "#,
        group_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorMinimal::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_maybe_flavor_detail_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Option<FlavorDetailed>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    pub struct FlavorDb {
        pub id: i32,
        pub name: String,
        pub openstack_id: String, // UUIDv4
        pub group_id: Option<i32>,
        pub group_name: Option<String>,
        pub weight: u32,
    }
    let query = sqlx::query!(
        r#"
        SELECT
            f.id AS id,
            f.name AS name,
            f.openstack_id AS openstack_id,
            g.id AS group_id,
            g.name AS group_name,
            f.weight AS weight
        FROM resources_flavor AS f
        LEFT JOIN resources_flavorgroup AS g
        ON f.group_id = g.id
        WHERE f.id = ?
        "#,
        flavor_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    let flavor = match row {
        Some(row) => {
            FlavorDb::from_row(&row).context("Failed to parse flavor row")?
        }
        None => return Ok(None),
    };
    Ok(Some(FlavorDetailed {
        id: flavor.id as u32,
        name: flavor.name,
        openstack_id: flavor.openstack_id,
        group: match (flavor.group_id, flavor.group_name.clone()) {
            (Some(id), Some(name)) => Some(FlavorGroupMinimal {
                id: id as u32,
                name,
            }),
            _ => None,
        },
        group_name: flavor.group_name,
        weight: flavor.weight,
    }))
}

#[tracing::instrument(name = "select_flavor_detail_from_db", skip(transaction))]
pub async fn select_flavor_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<FlavorDetailed, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_detail_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_maybe_lrz_flavor_detail_from_db",
    skip(transaction)
)]
pub async fn select_maybe_lrz_flavor_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: u64,
) -> Result<Option<FlavorDetailed>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    pub struct FlavorDb {
        pub id: i32,
        pub name: String,
        pub openstack_id: String, // UUIDv4
        pub group_id: Option<i32>,
        pub group_name: Option<String>,
        pub weight: u32,
    }
    let query = sqlx::query!(
        r#"
        SELECT
            f.id AS id,
            f.name AS name,
            f.openstack_id AS openstack_id,
            g.id AS group_id,
            g.name AS group_name,
            f.weight AS weight
        FROM resources_flavor AS f
        LEFT JOIN resources_flavorgroup AS g
        ON f.group_id = g.id
        WHERE
            f.id = ? AND
            g.name like 'lrz.%'
        "#,
        flavor_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    let flavor = match row {
        Some(row) => {
            FlavorDb::from_row(&row).context("Failed to parse flavor row")?
        }
        None => return Ok(None),
    };
    Ok(Some(FlavorDetailed {
        id: flavor.id as u32,
        name: flavor.name,
        openstack_id: flavor.openstack_id,
        group: match (flavor.group_id, flavor.group_name.clone()) {
            (Some(id), Some(name)) => Some(FlavorGroupMinimal {
                id: id as u32,
                name,
            }),
            _ => None,
        },
        group_name: flavor.group_name,
        weight: flavor.weight,
    }))
}

#[tracing::instrument(
    name = "select_lrz_flavor_detail_from_db",
    skip(transaction)
)]
pub async fn select_lrz_flavor_detail_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<FlavorDetailed, NotFoundOrUnexpectedApiError> {
    select_maybe_lrz_flavor_detail_from_db(transaction, user_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(name = "select_all_flavors_from_db", skip(transaction))]
pub async fn select_all_flavors_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<Flavor>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            f.id as id,
            f.name as name,
            f.openstack_id as openstack_id,
            f.group_id as group_id,
            g.name as group_name,
            f.weight as weight
        FROM resources_flavor as f
        LEFT JOIN resources_flavorgroup AS g
        ON f.group_id = g.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Flavor::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor")?;
    Ok(rows)
}

#[tracing::instrument(name = "select_lrz_flavors_from_db", skip(transaction))]
pub async fn select_lrz_flavors_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<Flavor>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            f.id as id,
            f.name as name,
            f.openstack_id as openstack_id,
            f.group_id as group_id,
            g.name as group_name,
            f.weight as weight
        FROM resources_flavorgroup as g, resources_flavor as f
        WHERE
            g.id = f.group_id AND
            g.name like 'lrz.%'
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Flavor::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "select_flavors_by_flavor_group_from_db",
    skip(transaction)
)]
pub async fn select_flavors_by_flavor_group_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_group_id: u64,
) -> Result<Vec<Flavor>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            f.id as id,
            f.name as name,
            f.openstack_id as openstack_id,
            f.group_id as group_id,
            g.name as group_name,
            f.weight as weight
        FROM resources_flavorgroup as g, resources_flavor as f
        WHERE
            g.id = f.group_id AND
            g.id = ?
        "#,
        flavor_group_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| Flavor::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor")?;
    Ok(rows)
}

#[tracing::instrument(
    name = "insert_flavor_into_db",
    skip(new_flavor, transaction)
)]
pub async fn insert_flavor_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_flavor: &FlavorCreateData,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO resources_flavor (name, openstack_id, weight, group_id)
        VALUES (?, ?, ?, ?)
        "#,
        new_flavor.name,
        new_flavor.openstack_id,
        new_flavor.weight,
        new_flavor.group,
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    // TODO: what about non-existing project_id?
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new flavor, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
