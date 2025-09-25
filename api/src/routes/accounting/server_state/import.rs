use std::{collections::HashMap, hash::Hash};

use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use anyhow::{Context, anyhow};
use avina_wire::{accounting::ServerStateImport, user::User};
use chrono::Utc;
use sqlx::{Executor, FromRow, MySql, MySqlPool, Transaction};

use crate::{
    authorization::require_admin_user,
    database::accounting::server_state::{
        NewServerState, insert_server_state_into_db,
        select_unfinished_server_states_from_db,
    },
    error::{
        NotFoundOrUnexpectedApiError, OptionApiError, UnexpectedOnlyError,
    },
    openstack::{OpenStack, ServerDetailed},
};

// NOTE: the hashmap cannot contain (None, None).
fn union_hash_zip<K, V, W>(
    hm1: HashMap<K, V>,
    hm2: HashMap<K, W>,
) -> HashMap<K, (Option<V>, Option<W>)>
where
    K: Clone + Hash + Eq,
    V: Clone + Sized,
    W: Clone + Sized,
{
    let mut hm3 = HashMap::new();
    for (k, v) in hm1.iter() {
        hm3.insert(k.clone(), (Some(v.clone()), None));
    }
    for (k, w) in hm2.iter() {
        hm3.entry(k.clone())
            .and_modify(|(_, u)| *u = Some(w.clone()))
            .or_insert((None, Some(w.clone())));
    }
    hm3
}

#[tracing::instrument(name = "server_state_import", skip(openstack))]
pub async fn server_state_import(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    openstack: Data<OpenStack>,
    // TODO: is the NormalApiError::ValidationError used?
    // Maybe we need a AuthOrUnexpectedError type.
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;

    // TODO: should we add additional context to the error here?
    let servers = openstack
        .get_servers()
        .await?
        .iter()
        .cloned()
        .map(|s| (s.id, s))
        .collect::<HashMap<_, _>>();
    let states = select_unfinished_server_states_from_db(&mut transaction)
        .await?
        .iter()
        .cloned()
        .map(|s| (s.instance_id, s))
        .collect::<HashMap<_, _>>();

    let servers_and_states = union_hash_zip(servers, states);

    let mut new_state_count = 0;
    let mut end_state_count = 0;

    for server_and_state in servers_and_states.values() {
        match server_and_state {
            (Some(server), Some(state)) => {
                if server.status != state.status {
                    end_server_state_in_db(&mut transaction, state.id as u64)
                        .await?;
                    end_state_count += 1;
                    new_state_count +=
                        create_server_state_in_db(&mut transaction, server)
                            .await?;
                }
            }
            (Some(server), None) => {
                new_state_count +=
                    create_server_state_in_db(&mut transaction, server).await?;
            }
            (None, Some(state)) => {
                end_server_state_in_db(&mut transaction, state.id as u64)
                    .await?;
                end_state_count += 1;
            }
            (None, None) => {
                return Err(anyhow!(
                    "Server state hash map contains invalid none-none pair."
                )
                .into());
            }
        }
    }

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok().content_type("application/json").json(
        ServerStateImport {
            new_state_count,
            end_state_count,
        },
    ))
}

#[tracing::instrument(name = "end_server_state_in_db", skip(transaction))]
pub async fn end_server_state_in_db(
    transaction: &mut Transaction<'_, MySql>,
    server_state_id: u64,
) -> Result<(), NotFoundOrUnexpectedApiError> {
    let query = sqlx::query!(
        r#"
        UPDATE accounting_state
        SET
            end = ?
        WHERE id = ?
        "#,
        Utc::now(),
        server_state_id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update first query")?;
    Ok(())
}

#[tracing::instrument(name = "create_server_state_in_db", skip(transaction))]
pub async fn create_server_state_in_db(
    transaction: &mut Transaction<'_, MySql>,
    server: &ServerDetailed,
) -> Result<u32, OptionApiError> {
    let Some(flavor_id) = select_maybe_flavor_id_by_openstack_id_from_db(
        transaction,
        server.flavor.id.clone(),
    )
    .await?
    else {
        tracing::warn!(
            "Flavor {} not found, skipping server state creation.",
            server.flavor.id.clone()
        );
        return Ok(0);
    };
    let Some(user_id) = select_maybe_user_id_by_openstack_id_from_db(
        transaction,
        server.tenant_id.clone(),
    )
    .await?
    else {
        tracing::warn!(
            "User {} not found, skipping server state creation.",
            server.tenant_id.clone()
        );
        return Ok(0);
    };
    let server_state = NewServerState {
        begin: Utc::now(),
        end: None,
        instance_id: server.id,
        instance_name: server.name.clone(),
        flavor: flavor_id as u32,
        status: server.status.clone(),
        user: user_id as u32,
    };
    let _ = insert_server_state_into_db(transaction, &server_state).await?;
    Ok(1)
}

#[tracing::instrument(
    name = "select_maybe_flavor_id_by_openstack_id_from_db",
    skip(transaction)
)]
pub async fn select_maybe_flavor_id_by_openstack_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_openstack_id: String,
) -> Result<Option<u64>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        #[sqlx(try_from = "i64")]
        id: u64,
    }
    let query = sqlx::query!(
        r#"
        SELECT id
        FROM resources_flavor AS flavor
        WHERE flavor.openstack_id = ?
        "#,
        flavor_openstack_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse flavor row")?
                .id,
        ),
        None => None,
    })
}

#[tracing::instrument(
    name = "select_flavor_id_by_openstack_id_from_db",
    skip(transaction)
)]
pub async fn select_flavor_id_by_openstack_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    flavor_id: String,
) -> Result<u64, NotFoundOrUnexpectedApiError> {
    select_maybe_flavor_id_by_openstack_id_from_db(transaction, flavor_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

#[tracing::instrument(
    name = "select_maybe_user_id_by_openstack_id_from_db",
    skip(transaction)
)]
pub async fn select_maybe_user_id_by_openstack_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_openstack_id: String,
) -> Result<Option<u64>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct Row {
        #[sqlx(try_from = "i64")]
        id: u64,
    }
    let query = sqlx::query!(
        r#"
        SELECT id
        FROM user_user AS user
        WHERE user.openstack_id = ?
        "#,
        user_openstack_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => {
            Some(Row::from_row(&row).context("Failed to parse user row")?.id)
        }
        None => None,
    })
}

#[tracing::instrument(
    name = "select_user_id_by_openstack_id_from_db",
    skip(transaction)
)]
pub async fn select_user_id_by_openstack_id_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_openstack_id: String,
) -> Result<u64, NotFoundOrUnexpectedApiError> {
    select_maybe_user_id_by_openstack_id_from_db(transaction, user_openstack_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}
