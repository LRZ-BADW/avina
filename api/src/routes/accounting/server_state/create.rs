use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::{
    accounting::{ServerState, ServerStateCreateData},
    user::User,
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::{
        accounting::server_state::{
            NewServerState, insert_server_state_into_db,
        },
        resources::flavor::select_flavor_name_from_db,
        user::user::select_user_name_from_db,
    },
    error::{NormalApiError, OptionApiError},
};

#[tracing::instrument(name = "server_state_create")]
pub async fn server_state_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<ServerStateCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let new_server_state: NewServerState = data
        .clone()
        .try_into()
        .map_err(NormalApiError::ValidationError)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let username = select_user_name_from_db(
        &mut transaction,
        new_server_state.user as u64,
    )
    .await?;
    let flavor_name = select_flavor_name_from_db(
        &mut transaction,
        new_server_state.flavor as u64,
    )
    .await?;
    let id = insert_server_state_into_db(&mut transaction, &new_server_state)
        .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let server_state_created = ServerState {
        id: id as u32,
        begin: data.begin,
        end: data.end,
        instance_id: new_server_state.instance_id,
        instance_name: new_server_state.instance_name.clone(),
        flavor: new_server_state.flavor,
        flavor_name,
        status: new_server_state.status.clone(),
        user: new_server_state.user,
        username,
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(server_state_created))
}
