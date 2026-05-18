//! Queries for server states.

use std::str::FromStr;

use anyhow::Context;
use avina_wire::{
    accounting::{ServerState, ServerStateCreateData},
    user::UserClass,
};
use chrono::{DateTime, Utc};
use sqlx::{Executor, FromRow, MySql, Transaction, types::uuid};
use uuid::Uuid;

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

/// Representation of a server state specifically for communication with the database.
///
/// This uses types, that can be directly deserialized from SQL and is then converted
/// to [ServerState] afterwards.
#[derive(FromRow)]
pub struct ServerStateRow {
    /// ID for the server state.
    #[sqlx(try_from = "i32")]
    pub id: u32,
    /// Timestamp when the server was first observed in this state.
    pub begin: DateTime<Utc>,
    /// Optional timestamp when the server was first observed having left this state.
    ///
    /// This is optional, as the server may still be in this state.
    pub end: Option<DateTime<Utc>>,
    /// UUID of the OpenStack server/instance.
    pub instance_id: String,
    /// Name of the OpenStack server/instance.
    pub instance_name: String,
    /// ID of the flavor during this state.
    #[sqlx(try_from = "i64")]
    pub flavor: u32,
    /// Name of the flavor during this state.
    pub flavor_name: String,
    /// Status during this state (ACTIVE, SHELVED_OFFLOADED, ...)
    pub status: String,
    /// ID of the user the server belongs to.
    #[sqlx(try_from = "i32")]
    pub user: u32,
    /// Name of the user the server belongs to.
    pub username: String,
}

/// Select a server state by the given ID from the database, or return [None].
#[tracing::instrument(
    name = "select_maybe_server_state_from_db",
    skip(transaction)
)]
pub async fn select_maybe_server_state_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_state_id: u64,
) -> Result<Option<ServerState>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id AND
            s.id = ?
        "#,
        server_state_id
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;
    Ok(match row {
        Some(row) => {
            let row = ServerStateRow::from_row(&row)
                .context("Failed to parse server state row")?;
            Some(ServerState {
                id: row.id,
                begin: row.begin.fixed_offset(),
                end: row.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(row.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: row.instance_name,
                flavor: row.flavor,
                flavor_name: row.flavor_name,
                status: row.status,
                user: row.user,
                username: row.username,
            })
        }
        None => None,
    })
}

/// Select a server state with the given ID from the database, or a "not found" error.
///
/// This calls [select_maybe_server_state_from_db] and then turns a [None] response into a
/// [NotFoundOrUnexpectedApiError::NotFoundError].
#[tracing::instrument(name = "select_server_state_from_db", skip(transaction))]
pub async fn select_server_state_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_state_id: u64,
) -> Result<ServerState, NotFoundOrUnexpectedApiError> {
    select_maybe_server_state_from_db(transaction, server_state_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

/// Select a list of all server states from the database.
#[tracing::instrument(
    name = "select_all_server_states_from_db",
    skip(transaction)
)]
pub async fn select_all_server_states_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}

/// Select a list of server states belonging to the project with the given ID from the database.
#[tracing::instrument(
    name = "select_server_states_by_project_from_db",
    skip(transaction)
)]
pub async fn select_server_states_by_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id AND
            u.project_id = ?
        "#,
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}

/// Select a list of server states belonging to the user with the given ID from the database.
#[tracing::instrument(
    name = "select_server_states_by_user_from_db",
    skip(transaction)
)]
pub async fn select_server_states_by_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id AND
            u.id = ?
        "#,
        user_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}

/// Select a single or entire list of server states belonging to the server with the given UUID from
/// the database.
///
/// In some cases, like authorization checks, it might suffice to get a single state instead of the
/// entire list.
#[tracing::instrument(
    name = "select_server_states_by_server_from_db",
    skip(transaction)
)]
pub async fn select_server_states_by_server_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: Uuid,
    fetch_one: bool,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id AND
            ss.instance_id = ?
        "#,
        server_id.to_string()
    );
    let queried_rows = if fetch_one {
        let row = transaction
            .fetch_one(query)
            .await
            .context("Failed to execute select query")?;
        vec![row]
    } else {
        transaction
            .fetch_all(query)
            .await
            .context("Failed to execute select query")?
    };

    let rows = queried_rows
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}

/// Select the user class of the project the server with the given UUID belongs to from the
/// database, or return [None].
#[tracing::instrument(
    name = "select_user_class_by_server_from_db",
    skip(transaction)
)]
pub async fn select_user_class_by_server_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: Uuid,
) -> Result<Option<UserClass>, UnexpectedOnlyError> {
    #[derive(FromRow)]
    struct Row {
        user_class: u32,
    }
    let query = sqlx::query!(
        r#"
        SELECT
            p.user_class as user_class
        FROM
            accounting_serverstate as ss,
            user_user as u,
            user_project as p
        WHERE
            ss.user_id = u.id AND
            u.project_id = p.id AND
            ss.instance_id = ?
        LIMIT 1
        "#,
        server_id.to_string(),
    );
    let row = transaction
        .fetch_optional(query)
        .await
        .context("Failed to execute select query")?;

    Ok(match row {
        Some(row) => Some(
            Row::from_row(&row)
                .context("Failed to parse user class row")?
                .user_class
                .try_into()
                .context("Failed to parse user class")?,
        ),
        None => None,
    })
}

/// Select a list of server states belonging to the server with the given UUID, while it was part of
/// the project with the given ID, from the database.
///
/// This is necessary, as users and therefore also their servers may switch projects.
#[tracing::instrument(
    name = "select_server_states_by_server_and_project_from_db",
    skip(transaction)
)]
pub async fn select_server_states_by_server_and_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: Uuid,
    project_id: u64,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id AND
            ss.instance_id = ? AND
            u.project_id = ?
        "#,
        server_id.to_string(),
        project_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}

/// Select a list of server states belonging to the server with the given UUID, while it belongs to
/// the user with the given ID, from the database.
///
/// This is necessary, as ownership to servers might change.
#[tracing::instrument(
    name = "select_server_states_by_server_and_user_from_db",
    skip(transaction)
)]
pub async fn select_server_states_by_server_and_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: Uuid,
    user_id: u64,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id AND
            ss.instance_id = ? AND
            u.id = ?
        "#,
        server_id.to_string(),
        user_id
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}

/// Simplified representation of data needed to create a new server state.
pub struct NewServerState {
    /// Timestamp when the server was first observed in this state.
    pub begin: DateTime<Utc>,
    /// Optional timestamp when the server was first observed having left this state.
    ///
    /// This is optional, as the server may still be in this state.
    pub end: Option<DateTime<Utc>>,
    /// UUID of the OpenStack server/instance.
    pub instance_id: Uuid,
    /// Name of the OpenStack server/instance.
    pub instance_name: String,
    /// ID of the flavor during this state.
    pub flavor: u32,
    /// Status during this state (ACTIVE, SHELVED_OFFLOADED, ...)
    // TODO: we need an enum here
    pub status: String,
    /// ID of the user the server belongs to.
    pub user: u32,
}

// TODO: really validate data
impl TryFrom<ServerStateCreateData> for NewServerState {
    type Error = String;

    /// Transform a [ServerStateCreateData] into a [NewServerState].
    ///
    /// More specifically this only transforms begin and end to UTC.
    fn try_from(data: ServerStateCreateData) -> Result<Self, Self::Error> {
        Ok(Self {
            begin: data.begin.to_utc(),
            end: data.end.map(|d| d.to_utc()),
            instance_id: data.instance_id,
            instance_name: data.instance_name,
            flavor: data.flavor,
            status: data.status,
            user: data.user,
        })
    }
}

/// Insert a new server state based on the given [NewServerState] into the database.
#[tracing::instrument(
    name = "insert_server_state_into_db",
    skip(new_server_state, transaction)
)]
pub async fn insert_server_state_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_server_state: &NewServerState,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query1 = sqlx::query!(
        r#"
        INSERT IGNORE INTO accounting_state (begin, end)
        VALUES (?, ?)
        "#,
        new_server_state.begin,
        new_server_state.end,
    );
    let result1 = transaction
        .execute(query1)
        .await
        .context("Failed to execute insert query")?;
    if result1.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new state, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result1.last_insert_id();
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query2 = sqlx::query!(
        r#"
        INSERT IGNORE INTO accounting_serverstate (
            state_ptr_id, instance_id, instance_name, status, flavor_id, user_id
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        id,
        new_server_state.instance_id.to_string(),
        new_server_state.instance_name,
        new_server_state.status,
        new_server_state.flavor,
        new_server_state.user
    );
    let result2 = transaction
        .execute(query2)
        .await
        .context("Failed to execute insert query")?;
    if result2.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new server state, a conflicting entry exists"
                .to_string(),
        ));
    }
    Ok(id)
}

/// Select server states from the database for the server with the given UUID, that where active
/// between the optionally given begin and end timestamps, ordered by their ID and thus also begin.
///
/// If only begin is given, all states active afterwards are returned, and if only end is given, all
/// states active before are returned. If neither is given, then all states are returned for the
/// server.
#[tracing::instrument(
    name = "select_ordered_server_states_by_server_begin_and_end_from_db",
    skip(transaction)
)]
pub async fn select_ordered_server_states_by_server_begin_and_end_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: Uuid,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let result = match (begin, end) {
        (None, None) => {
            let query = sqlx::query!(
                r#"
                SELECT
                    s.id as id,
                    s.begin as begin,
                    s.end as end,
                    ss.instance_id as instance_id,
                    ss.instance_name as instance_name,
                    f.id as flavor,
                    f.name as flavor_name,
                    ss.status as status,
                    u.id as user,
                    u.name as username
                FROM
                    accounting_state as s,
                    accounting_serverstate as ss,
                    resources_flavor as f,
                    user_user as u
                WHERE
                    ss.flavor_id = f.id AND
                    ss.user_id = u.id AND
                    ss.state_ptr_id = s.id AND
                    ss.instance_id = ?
                ORDER BY s.id
                "#,
                server_id.to_string(),
            );
            transaction.fetch_all(query).await
        }
        (Some(begin), None) => {
            let query = sqlx::query!(
                r#"
                SELECT
                    s.id as id,
                    s.begin as begin,
                    s.end as end,
                    ss.instance_id as instance_id,
                    ss.instance_name as instance_name,
                    f.id as flavor,
                    f.name as flavor_name,
                    ss.status as status,
                    u.id as user,
                    u.name as username
                FROM
                    accounting_state as s,
                    accounting_serverstate as ss,
                    resources_flavor as f,
                    user_user as u
                WHERE
                    ss.flavor_id = f.id AND
                    ss.user_id = u.id AND
                    ss.state_ptr_id = s.id AND
                    ss.instance_id = ? AND
                    (s.end > ? OR s.end IS NULL)
                ORDER BY s.id
                "#,
                server_id.to_string(),
                begin
            );
            transaction.fetch_all(query).await
        }
        (None, Some(end)) => {
            let query = sqlx::query!(
                r#"
                SELECT
                    s.id as id,
                    s.begin as begin,
                    s.end as end,
                    ss.instance_id as instance_id,
                    ss.instance_name as instance_name,
                    f.id as flavor,
                    f.name as flavor_name,
                    ss.status as status,
                    u.id as user,
                    u.name as username
                FROM
                    accounting_state as s,
                    accounting_serverstate as ss,
                    resources_flavor as f,
                    user_user as u
                WHERE
                    ss.flavor_id = f.id AND
                    ss.user_id = u.id AND
                    ss.state_ptr_id = s.id AND
                    ss.instance_id = ? AND
                    s.begin < ?
                ORDER BY s.id
                "#,
                server_id.to_string(),
                end
            );
            transaction.fetch_all(query).await
        }
        (Some(begin), Some(end)) => {
            let query = sqlx::query!(
                r#"
                SELECT
                    s.id as id,
                    s.begin as begin,
                    s.end as end,
                    ss.instance_id as instance_id,
                    ss.instance_name as instance_name,
                    f.id as flavor,
                    f.name as flavor_name,
                    ss.status as status,
                    u.id as user,
                    u.name as username
                FROM
                    accounting_state as s,
                    accounting_serverstate as ss,
                    resources_flavor as f,
                    user_user as u
                WHERE
                    ss.flavor_id = f.id AND
                    ss.user_id = u.id AND
                    ss.state_ptr_id = s.id AND
                    ss.instance_id = ? AND
                    (s.end > ? OR s.end IS NULL) AND
                    s.begin < ?
                ORDER BY s.id
                "#,
                server_id.to_string(),
                begin,
                end
            );
            transaction.fetch_all(query).await
        }
    };
    let rows = result
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}

/// Select server states from the database belonging to the user with the given ID, that where
/// active between the optionally given begin and end timestamps, ordered by their ID and thus also
/// begin.
///
/// If only begin is given, all states active afterwards are returned, and if only end is given, all
/// states active before are returned. If neither is given, then all states are returned for the
/// server.
#[tracing::instrument(
    name = "select_ordered_server_states_by_user_begin_and_end_from_db",
    skip(transaction)
)]
pub async fn select_ordered_server_states_by_user_begin_and_end_from_db(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let result = match (begin, end) {
        (None, None) => {
            let query = sqlx::query!(
                r#"
                SELECT
                    s.id as id,
                    s.begin as begin,
                    s.end as end,
                    ss.instance_id as instance_id,
                    ss.instance_name as instance_name,
                    f.id as flavor,
                    f.name as flavor_name,
                    ss.status as status,
                    u.id as user,
                    u.name as username
                FROM
                    accounting_state as s,
                    accounting_serverstate as ss,
                    resources_flavor as f,
                    user_user as u
                WHERE
                    ss.flavor_id = f.id AND
                    ss.user_id = u.id AND
                    ss.state_ptr_id = s.id AND
                    ss.user_id = ?
                ORDER BY s.id
                "#,
                user_id
            );
            transaction.fetch_all(query).await
        }
        (Some(begin), None) => {
            let query = sqlx::query!(
                r#"
                SELECT
                    s.id as id,
                    s.begin as begin,
                    s.end as end,
                    ss.instance_id as instance_id,
                    ss.instance_name as instance_name,
                    f.id as flavor,
                    f.name as flavor_name,
                    ss.status as status,
                    u.id as user,
                    u.name as username
                FROM
                    accounting_state as s,
                    accounting_serverstate as ss,
                    resources_flavor as f,
                    user_user as u
                WHERE
                    ss.flavor_id = f.id AND
                    ss.user_id = u.id AND
                    ss.state_ptr_id = s.id AND
                    ss.user_id = ? AND
                    (s.end > ? OR s.end IS NULL)
                ORDER BY s.id
                "#,
                user_id,
                begin
            );
            transaction.fetch_all(query).await
        }
        (None, Some(end)) => {
            let query = sqlx::query!(
                r#"
                SELECT
                    s.id as id,
                    s.begin as begin,
                    s.end as end,
                    ss.instance_id as instance_id,
                    ss.instance_name as instance_name,
                    f.id as flavor,
                    f.name as flavor_name,
                    ss.status as status,
                    u.id as user,
                    u.name as username
                FROM
                    accounting_state as s,
                    accounting_serverstate as ss,
                    resources_flavor as f,
                    user_user as u
                WHERE
                    ss.flavor_id = f.id AND
                    ss.user_id = u.id AND
                    ss.state_ptr_id = s.id AND
                    ss.user_id = ? AND
                    s.begin < ?
                ORDER BY s.id
                "#,
                user_id,
                end
            );
            transaction.fetch_all(query).await
        }
        (Some(begin), Some(end)) => {
            let query = sqlx::query!(
                r#"
                SELECT
                    s.id as id,
                    s.begin as begin,
                    s.end as end,
                    ss.instance_id as instance_id,
                    ss.instance_name as instance_name,
                    f.id as flavor,
                    f.name as flavor_name,
                    ss.status as status,
                    u.id as user,
                    u.name as username
                FROM
                    accounting_state as s,
                    accounting_serverstate as ss,
                    resources_flavor as f,
                    user_user as u
                WHERE
                    ss.flavor_id = f.id AND
                    ss.user_id = u.id AND
                    ss.state_ptr_id = s.id AND
                    ss.user_id = ? AND
                    (s.end > ? OR s.end IS NULL) AND
                    s.begin < ?
                ORDER BY s.id
                "#,
                user_id,
                begin,
                end
            );
            transaction.fetch_all(query).await
        }
    };
    let rows = result
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}

/// Select all not yet completed server states from the database.
///
/// This means all server states without an end timestamp. Note, that this should always only be one
/// per server.
#[tracing::instrument(
    name = "select_unfinished_server_states_from_db",
    skip(transaction)
)]
pub async fn select_unfinished_server_states_from_db(
    transaction: &mut Transaction<'_, MySql>,
) -> Result<Vec<ServerState>, UnexpectedOnlyError> {
    let query = sqlx::query!(
        r#"
        SELECT
            s.id as id,
            s.begin as begin,
            s.end as end,
            ss.instance_id as instance_id,
            ss.instance_name as instance_name,
            f.id as flavor,
            f.name as flavor_name,
            ss.status as status,
            u.id as user,
            u.name as username
        FROM
            accounting_state as s,
            accounting_serverstate as ss,
            resources_flavor as f,
            user_user as u
        WHERE
            ss.flavor_id = f.id AND
            ss.user_id = u.id AND
            ss.state_ptr_id = s.id AND
            s.end is NULL
        "#,
    );
    let rows = transaction
        .fetch_all(query)
        .await
        .context("Failed to execute select query")?
        .into_iter()
        .map(|r| ServerStateRow::from_row(&r))
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert row to server state")?
        .into_iter()
        .map(|r| {
            Ok::<ServerState, UnexpectedOnlyError>(ServerState {
                id: r.id,
                begin: r.begin.fixed_offset(),
                end: r.end.map(|end| end.fixed_offset()),
                instance_id: Uuid::from_str(r.instance_id.as_str())
                    .context("Could not parse instance id String")?,
                instance_name: r.instance_name,
                flavor: r.flavor,
                flavor_name: r.flavor_name,
                status: r.status,
                user: r.user,
                username: r.username,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to convert server state row to server state")?;
    Ok(rows)
}
