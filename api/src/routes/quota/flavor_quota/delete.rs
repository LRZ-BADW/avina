use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::FlavorQuotaIdParam;
use crate::{
    authorization::require_admin_user,
    database::quota::flavor_quota::delete_flavor_quota_from_db,
    error::NormalApiError,
};

#[tracing::instrument(name = "flavor_quota_delete")]
pub async fn flavor_quota_delete(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorQuotaIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_flavor_quota_from_db(
        &mut transaction,
        params.flavor_quota_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}
