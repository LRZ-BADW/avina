use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    accounting::ServerStateListParams,
    user::{Project, User},
};
use sqlx::MySqlPool;

use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
        require_user_or_project_master_or_not_found,
    },
    database::{
        accounting::server_state::{
            select_all_server_states_from_db,
            select_server_states_by_project_from_db,
            select_server_states_by_server_and_project_from_db,
            select_server_states_by_server_and_user_from_db,
            select_server_states_by_server_from_db,
            select_server_states_by_user_from_db,
        },
        user::user::select_user_from_db,
    },
    error::OptionApiError,
};

#[tracing::instrument(name = "server_state_list")]
pub async fn server_state_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ServerStateListParams>,
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let server_states = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_server_states_from_db(&mut transaction).await?
    } else if let Some(project_id) = params.project {
        require_master_user_or_return_not_found(&user, project_id)?;
        if let Some(server_id) = params.server {
            select_server_states_by_server_and_project_from_db(
                &mut transaction,
                server_id,
                project_id as u64,
            )
            .await?
        } else {
            select_server_states_by_project_from_db(
                &mut transaction,
                project_id as u64,
            )
            .await?
        }
    } else if let Some(user_id) = params.user {
        let user1 = select_user_from_db(&mut transaction, user_id as u64)
            .await
            .context("Failed to select user")?;
        require_user_or_project_master_or_not_found(
            &user,
            user1.id,
            user1.project,
        )?;
        if let Some(server_id) = params.server {
            select_server_states_by_server_and_user_from_db(
                &mut transaction,
                server_id,
                user1.id as u64,
            )
            .await?
        } else {
            select_server_states_by_user_from_db(
                &mut transaction,
                user1.id as u64,
            )
            .await?
        }
    } else if let Some(server_id) = params.server {
        let server_states = select_server_states_by_server_from_db(
            &mut transaction,
            server_id,
            false,
        )
        .await?;
        let server_state_user =
            select_user_from_db(&mut transaction, server_states[0].user as u64)
                .await?;
        match require_user_or_project_master_or_not_found(
            &user,
            server_state_user.id,
            server_state_user.project,
        ) {
            Ok(_) => server_states,
            Err(e) => return Err(e.into()),
        }
    } else {
        select_server_states_by_user_from_db(&mut transaction, user.id as u64)
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(server_states))
}
