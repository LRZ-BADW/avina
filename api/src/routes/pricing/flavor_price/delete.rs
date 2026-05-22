use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::FlavorPriceIdParam;
use crate::{
    authorization::require_admin_user,
    database::pricing::flavor_price::delete_flavor_price_from_db,
    error::NormalApiError,
};

#[tracing::instrument(name = "flavor_price_delete")]
pub async fn flavor_price_delete(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorPriceIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_flavor_price_from_db(
        &mut transaction,
        params.flavor_price_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}
