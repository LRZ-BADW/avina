use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::{quota::FlavorQuotaModifyData, user::User};
use sqlx::MySqlPool;

use super::FlavorQuotaIdParam;
use crate::{
    authorization::require_admin_user,
    database::quota::flavor_quota::update_flavor_quota_in_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "flavor_quota_modify")]
pub async fn flavor_quota_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorQuotaModifyData>,
    params: Path<FlavorQuotaIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.flavor_quota_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_quota =
        update_flavor_quota_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_quota))
}
