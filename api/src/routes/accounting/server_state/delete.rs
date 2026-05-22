use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::ServerStateIdParam;
use crate::{
    authorization::require_admin_user,
    database::accounting::server_state::delete_server_state_from_db,
    error::NormalApiError,
};

#[tracing::instrument(name = "server_state_delete")]
pub async fn server_state_delete(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<ServerStateIdParam>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    delete_server_state_from_db(
        &mut transaction,
        params.server_state_id as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::NoContent().finish())
}
