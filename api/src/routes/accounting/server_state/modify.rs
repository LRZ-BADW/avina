use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::{accounting::ServerStateModifyData, user::User};
use sqlx::MySqlPool;

use super::ServerStateIdParam;
use crate::{
    authorization::require_admin_user,
    database::accounting::server_state::update_server_state_in_db,
    error::OptionApiError,
};

#[tracing::instrument(name = "server_state_modify")]
pub async fn server_state_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<ServerStateModifyData>,
    params: Path<ServerStateIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.server_state_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let server_state =
        update_server_state_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(server_state))
}
