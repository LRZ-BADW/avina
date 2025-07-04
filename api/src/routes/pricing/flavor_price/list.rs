use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, User};
use sqlx::MySqlPool;

use crate::{
    database::pricing::flavor_price::select_all_flavor_prices_from_db,
    error::NormalApiError,
};

#[tracing::instrument(name = "flavor_price_list")]
pub async fn flavor_price_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_prices =
        select_all_flavor_prices_from_db(&mut transaction).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_prices))
}
