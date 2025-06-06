use anyhow::Context;
use avina_wire::pricing::{FlavorPrice, FlavorPriceCreateData};
use chrono::{DateTime, Utc};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

#[derive(FromRow)]
pub struct FlavorPriceRow {
    #[sqlx(try_from = "i32")]
    pub id: u32,
    #[sqlx(try_from = "i32")]
    pub flavor: u32,
    pub flavor_name: String,
    pub user_class: u32,
    pub unit_price: f64,
    pub start_time: DateTime<Utc>,
}

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
                user_class: row.user_class,
                unit_price: row.unit_price,
                start_time: row.start_time.fixed_offset(),
            })
        }
        None => None,
    })
}

#[tracing::instrument(name = "select_flavor_price_from_db", skip(transaction))]
pub async fn select_flavor_price_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_price_id: u64,
) -> Result<FlavorPrice, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_price_from_db(transaction, flavor_price_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

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
        .context("Failed to convert row to flavor price")?
        .into_iter()
        .map(|row| FlavorPrice {
            id: row.id,
            flavor: row.flavor,
            flavor_name: row.flavor_name,
            user_class: row.user_class,
            unit_price: row.unit_price,
            start_time: row.start_time.fixed_offset(),
        })
        .collect();
    Ok(rows)
}

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
        .context("Failed to convert row to flavor price")?
        .into_iter()
        .map(|row| FlavorPrice {
            id: row.id,
            flavor: row.flavor,
            flavor_name: row.flavor_name,
            user_class: row.user_class,
            unit_price: row.unit_price,
            start_time: row.start_time.fixed_offset(),
        })
        .collect();
    Ok(rows)
}

pub struct NewFlavorPrice {
    pub flavor_id: u64,
    pub user_class: u32,
    pub unit_price: f64,
    pub start_time: DateTime<Utc>,
}

impl TryFrom<FlavorPriceCreateData> for NewFlavorPrice {
    type Error = String;

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
        new_flavor_price.user_class,
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
