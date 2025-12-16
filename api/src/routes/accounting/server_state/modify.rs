use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::{
    accounting::{ServerState, ServerStateModifyData},
    user::User,
};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::ServerStateIdParam;
use crate::{
    authorization::require_admin_user,
    database::{
        accounting::server_state::select_server_state_from_db,
        resources::flavor::select_flavor_name_from_db,
        user::user::select_user_name_from_db,
    },
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
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

#[tracing::instrument(
    name = "update_server_state_in_db",
    skip(data, transaction)
)]
pub async fn update_server_state_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &ServerStateModifyData,
) -> Result<ServerState, NotFoundOrUnexpectedApiError> {
    let row = select_server_state_from_db(transaction, data.id as u64).await?;
    let begin = data.begin.unwrap_or(row.begin);
    let mut end = data.end;
    if end.is_none() {
        end = row.end;
    }
    let instance_id = data.instance_id.unwrap_or(row.instance_id);
    let instance_name = data.instance_name.clone().unwrap_or(row.instance_name);
    let status = data.status.clone().unwrap_or(row.status);
    let user = data.user.unwrap_or(row.user);
    let username = select_user_name_from_db(transaction, user as u64).await?;
    let flavor = data.flavor.unwrap_or(row.flavor);
    let flavor_name =
        select_flavor_name_from_db(transaction, user as u64).await?;
    let query1 = sqlx::query!(
        r#"
        UPDATE accounting_state
        SET
            begin = ?,
            end = ?
        WHERE id = ?
        "#,
        begin.to_utc(),
        end.map(|end| end.to_utc()),
        data.id,
    );
    transaction
        .execute(query1)
        .await
        .context("Failed to execute update first query")?;
    let query2 = sqlx::query!(
        r#"
        UPDATE accounting_serverstate
        SET
            instance_id = ?,
            instance_name = ?,
            flavor_id = ?,
            status = ?,
            user_id = ?
        WHERE state_ptr_id = ?
        "#,
        instance_id.to_string(),
        instance_name,
        flavor,
        status,
        user,
        data.id,
    );
    transaction
        .execute(query2)
        .await
        .context("Failed to execute update second query")?;
    let price = ServerState {
        id: data.id,
        begin,
        end,
        instance_id,
        instance_name,
        flavor,
        flavor_name,
        status,
        user,
        username,
    };
    Ok(price)
}
