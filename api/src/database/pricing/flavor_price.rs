//! Queries for flavor prices.

use std::collections::HashMap;

use anyhow::Context;
use avina_wire::{
    pricing::{FlavorPrice, FlavorPriceCreateData},
    user::UserClass,
};
use chrono::{DateTime, Utc};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

/// Representation of a flavor price specifically for communication with the database.
///
/// This uses types, that can be directly deserialized from SQL and is then converted
/// to [FlavorPrice] afterwards.
#[derive(FromRow)]
pub struct FlavorPriceRow {
    /// ID of the flavor price.
    #[sqlx(try_from = "i32")]
    pub id: u32,
    /// ID of the flavor.
    #[sqlx(try_from = "i32")]
    pub flavor: u32,
    /// Name of the flavor.
    pub flavor_name: String,
    /// User class of the price.
    pub user_class: u32,
    /// Actual price per unit (full year usage of the flavor).
    pub unit_price: f64,
    /// Timestamp from when the price is valid.
    ///
    /// It is valid until the next price for the same flavor and user class takes over.
    pub start_time: DateTime<Utc>,
}

/// Select a flavor price by the given ID from the database, or return [None].
#[tracing::instrument(
    name = "select_maybe_flavor_price_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_price_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_price_id: u64,
) -> Result<Option<FlavorPrice>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            p.id,
            p.flavor_id as flavor,
            f.name as flavor_name, 
            p.user_class as user_class,
            p.unit_price as unit_price,
            p.start_time as start_time
        FROM
            pricing_flavorprice as p,
            resources_flavor as f
        WHERE
            p.flavor_id = f.id AND
            p.id = ?
        "#,
        flavor_price_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => {
            let row = FlavorPriceRow::from_row(&row)
                .context("Failed to parse flavor price row")?;
            Some(FlavorPrice {
                id: row.id,
                flavor: row.flavor,
                flavor_name: row.flavor_name,
                user_class: row
                    .user_class
                    .try_into()
                    .context("Failed to parse user class")?,
                unit_price: row.unit_price,
                start_time: row.start_time.fixed_offset(),
            })
        }
        None => None,
    })
}

/// Select a flavor price with the given ID from the database, or a "not found" error.
///
/// This calls [select_maybe_flavor_price_from_db] and then turns a [None] response into a
/// [NotFoundOrUnexpectedApiError::NotFoundError].
#[tracing::instrument(name = "select_flavor_price_from_db", skip(transaction))]
pub async fn select_flavor_price_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_price_id: u64,
) -> Result<FlavorPrice, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_price_from_db(transaction, flavor_price_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

/// Select a list of all flavor prices from the database.
#[tracing::instrument(
    name = "select_all_flavor_prices_from_db",
    skip(transaction)
)]
pub async fn select_all_flavor_prices_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<FlavorPrice>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            p.id,
            p.flavor_id as flavor,
            f.name as flavor_name, 
            p.user_class as user_class,
            p.unit_price as unit_price,
            p.start_time as start_time
        FROM
            pricing_flavorprice as p,
            resources_flavor as f
        WHERE
            p.flavor_id = f.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorPriceRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor price row")?
        .into_iter()
        .map(|row| {
            Ok::<FlavorPrice, UnexpectedOnlyError>(FlavorPrice {
                id: row.id,
                flavor: row.flavor,
                flavor_name: row.flavor_name,
                user_class: row
                    .user_class
                    .try_into()
                    .context("Failed to parse user class")?,
                unit_price: row.unit_price,
                start_time: row.start_time.fixed_offset(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert flavor price row to flavor price")?;
    Ok(rows)
}

/// Select a list of flavor prices that are valid during the period given by begin and end
/// timestamp.
#[tracing::instrument(
    name = "select_flavor_prices_for_period_from_db",
    skip(transaction)
)]
pub async fn select_flavor_prices_for_period_from_db(
    transaction: &mut Transaction<'_, MySql>,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<FlavorPrice>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            p.id,
            p.flavor_id as flavor,
            f.name as flavor_name, 
            p.user_class as user_class,
            p.unit_price as unit_price,
            p.start_time as start_time
        FROM
            pricing_flavorprice as p,
            resources_flavor as f
        WHERE
            p.flavor_id = f.id AND
            p.start_time <= ?
        ORDER BY p.start_time DESC
        "#,
        end,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorPriceRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor price row")?
        .into_iter()
        .map(|row| {
            Ok::<FlavorPrice, UnexpectedOnlyError>(FlavorPrice {
                id: row.id,
                flavor: row.flavor,
                flavor_name: row.flavor_name,
                user_class: row
                    .user_class
                    .try_into()
                    .context("Failed to parse user class")?,
                unit_price: row.unit_price,
                start_time: row.start_time.fixed_offset(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor price")?;
    let mut prices = Vec::new();
    let mut done = HashMap::new();
    for price in rows {
        let this_done = done
            .entry(price.flavor)
            .or_insert(HashMap::new())
            .entry(price.user_class)
            .or_insert(false);
        if *this_done {
            continue;
        }
        if price.start_time <= begin {
            *this_done = true;
        }
        prices.push(price);
    }
    Ok(prices)
}

/// Select a list of flavor prices with the given user class from the database.
#[tracing::instrument(
    name = "select_flavor_prices_for_userclass_from_db",
    skip(transaction)
)]
pub async fn select_flavor_prices_for_userclass_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_class: UserClass,
) -> Result<Vec<FlavorPrice>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            p.id,
            p.flavor_id as flavor,
            f.name as flavor_name,
            p.user_class as user_class,
            p.unit_price as unit_price,
            p.start_time as start_time
        FROM
            pricing_flavorprice as p,
            resources_flavor as f
        WHERE
            p.flavor_id = f.id AND
            p.user_class = ?
        "#,
        user_class as u32
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorPriceRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor price row")?
        .into_iter()
        .map(|row| {
            Ok::<FlavorPrice, UnexpectedOnlyError>(FlavorPrice {
                id: row.id,
                flavor: row.flavor,
                flavor_name: row.flavor_name,
                user_class: row
                    .user_class
                    .try_into()
                    .context("Failed to parse user class")?,
                unit_price: row.unit_price,
                start_time: row.start_time.fixed_offset(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor price")?;
    Ok(rows)
}

/// Select a list of flavor prices for a given user class, that are valid during the period given
/// by begin and end timestamp.
#[tracing::instrument(
    name = "select_flavor_prices_for_userclass_and_period_from_db",
    skip(transaction)
)]
pub async fn select_flavor_prices_for_userclass_and_period_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_class: UserClass,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<FlavorPrice>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            p.id,
            p.flavor_id as flavor,
            f.name as flavor_name, 
            p.user_class as user_class,
            p.unit_price as unit_price,
            p.start_time as start_time
        FROM
            pricing_flavorprice as p,
            resources_flavor as f
        WHERE
            p.flavor_id = f.id AND
            p.user_class = ? AND
            p.start_time <= ?
        ORDER BY p.start_time DESC
        "#,
        user_class as u32,
        end,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| FlavorPriceRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor price row")?
        .into_iter()
        .map(|row| {
            Ok::<FlavorPrice, UnexpectedOnlyError>(FlavorPrice {
                id: row.id,
                flavor: row.flavor,
                flavor_name: row.flavor_name,
                user_class: row
                    .user_class
                    .try_into()
                    .context("Failed to parse user class")?,
                unit_price: row.unit_price,
                start_time: row.start_time.fixed_offset(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to flavor price")?;
    let mut prices = Vec::new();
    let mut done = HashMap::new();
    for price in rows {
        let this_done = done
            .entry(price.flavor)
            .or_insert(HashMap::new())
            .entry(price.user_class)
            .or_insert(false);
        if *this_done {
            continue;
        }
        if price.start_time <= begin {
            *this_done = true;
        }
        prices.push(price);
    }
    Ok(prices)
}

/// Simplified representation of data needed to create a new flavor price.
pub struct NewFlavorPrice {
    /// ID of the flavor.
    pub flavor_id: u64,
    /// User class for the price.
    pub user_class: UserClass,
    /// Actual price per unit (full year usage of the flavor).
    pub unit_price: f64,
    /// Timestamp from when the price is valid.
    ///
    /// It is valid until the next price for the same flavor and user class takes over.
    pub start_time: DateTime<Utc>,
}

impl TryFrom<FlavorPriceCreateData> for NewFlavorPrice {
    type Error = String;

    /// Transform a [FlavorPriceCreateData] into a [NewFlavorPrice].
    ///
    /// More specifically this also replaces not inputted data by defaults, e.g.,
    /// 0. for the unit price, and now in UTC for the start time.
    fn try_from(data: FlavorPriceCreateData) -> Result<Self, Self::Error> {
        Ok(Self {
            flavor_id: data.flavor as u64,
            user_class: data.user_class,
            unit_price: data.price.unwrap_or(0.),
            start_time: data
                .start_time
                .map(|d| d.to_utc())
                .unwrap_or(Utc::now()),
        })
    }
}

/// Insert a new flavor price based on the given [NewFlavorPrice] into the database.
#[tracing::instrument(
    name = "insert_flavor_price_into_db",
    skip(new_flavor_price, transaction)
)]
pub async fn insert_flavor_price_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_flavor_price: &NewFlavorPrice,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO pricing_flavorprice (user_class, unit_price, start_time, flavor_id)
        VALUES (?, ?, ?, ?)
        "#,
        new_flavor_price.user_class as u32,
        new_flavor_price.unit_price,
        new_flavor_price.start_time,
        new_flavor_price.flavor_id,
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    // TODO: what about non-existing project_id?
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new flavor price, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
