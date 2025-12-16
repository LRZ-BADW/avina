use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::{
    pricing::{FlavorPrice, FlavorPriceModifyData},
    user::User,
};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::FlavorPriceIdParam;
use crate::{
    authorization::require_admin_user,
    database::{
        pricing::flavor_price::select_flavor_price_from_db,
        resources::flavor::select_flavor_name_from_db,
    },
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
};

#[tracing::instrument(name = "flavor_price_modify")]
pub async fn flavor_price_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorPriceModifyData>,
    params: Path<FlavorPriceIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.flavor_price_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_price =
        update_flavor_price_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_price))
}

#[tracing::instrument(
    name = "update_flavor_price_in_db",
    skip(data, transaction)
)]
pub async fn update_flavor_price_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &FlavorPriceModifyData,
) -> Result<FlavorPrice, NotFoundOrUnexpectedApiError> {
    let row = select_flavor_price_from_db(transaction, data.id as u64).await?;
    let user_class = data.user_class.unwrap_or(row.user_class);
    let unit_price = data.unit_price.unwrap_or(row.unit_price);
    let start_time = data.start_time.unwrap_or(row.start_time);
    let flavor = data.flavor.unwrap_or(row.flavor);
    let flavor_name =
        select_flavor_name_from_db(transaction, flavor as u64).await?;
    let query = sqlx::query!(
        r#"
        UPDATE pricing_flavorprice
        SET user_class = ?, unit_price = ?, start_time = ?, flavor_id = ?
        WHERE id = ?
        "#,
        user_class as u32,
        unit_price,
        start_time.to_utc(),
        flavor,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let price = FlavorPrice {
        id: data.id,
        user_class,
        unit_price,
        start_time,
        flavor,
        flavor_name,
    };
    Ok(price)
}
