use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    pricing::FlavorPriceListParams,
    user::{Project, User},
};
use chrono::Utc;
use sqlx::MySqlPool;

use crate::{
    database::pricing::flavor_price::{
        select_all_flavor_prices_from_db,
        select_flavor_prices_for_period_from_db,
        select_flavor_prices_for_userclass_and_period_from_db,
        select_flavor_prices_for_userclass_from_db,
    },
    error::NormalApiError,
};

#[tracing::instrument(name = "flavor_price_list")]
pub async fn flavor_price_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<FlavorPriceListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;

    let flavor_prices = match (params.user_class, params.current) {
        (Some(user_class), Some(true)) => {
            let now = Utc::now();
            select_flavor_prices_for_userclass_and_period_from_db(
                &mut transaction,
                user_class,
                now,
                now,
            )
            .await?
        }
        (Some(user_class), Some(false) | None) => {
            select_flavor_prices_for_userclass_from_db(
                &mut transaction,
                user_class,
            )
            .await?
        }
        (None, Some(true)) => {
            let now = Utc::now();
            select_flavor_prices_for_period_from_db(&mut transaction, now, now)
                .await?
        }
        _ => select_all_flavor_prices_from_db(&mut transaction).await?,
    };

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_prices))
}
