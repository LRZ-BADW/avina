use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::FlavorQuotaIdParam;
use crate::{
    database::quota::flavor_quota::select_flavor_quota_from_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "flavor_quota_get")]
pub async fn flavor_quota_get(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorQuotaIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_quota = select_flavor_quota_from_db(
        &mut transaction,
        params.flavor_quota_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    if flavor_quota.user != user.id && !user.is_staff {
        return Err(OptionApiError::NotFoundError);
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_quota))
}
