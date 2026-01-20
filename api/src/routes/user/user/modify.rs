use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::{User, UserModifyData};
use sqlx::MySqlPool;

use super::UserIdParam;
use crate::{
    authorization::require_admin_user, database::user::user::update_user_in_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "user_modify")]
pub async fn user_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<UserModifyData>,
    params: Path<UserIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.user_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let user = update_user_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user))
}
