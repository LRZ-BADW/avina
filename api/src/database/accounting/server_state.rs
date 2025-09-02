use anyhow::Context;
use avina_wire::{
    accounting::{ServerState, ServerStateCreateData},
    user::UserClass,
};
use chrono::{DateTime, Utc};
use sqlx::{Executor, FromRow, MySql, Transaction};

use crate::error::{
    MinimalApiError, NotFoundOrUnexpectedApiError, UnexpectedOnlyError,
};

#[derive(FromRow)]
pub struct ServerStateRow {
    #[sqlx(try_from = "i32")]
    pub id: u32,
    pub begin: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub instance_id: String,
    pub instance_name: String,
    #[sqlx(try_from = "i64")]
    pub flavor: u32,
    pub flavor_name: String,
    pub status: String,
    #[sqlx(try_from = "i32")]
    pub user: u32,
    pub username: String,
}

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
                .context("Failed to parse flavor price row")?;
            Some(ServerState {
                id: row.id,
                begin: row.begin.fixed_offset(),
                end: row.end.map(|end| end.fixed_offset()),
                instance_id: row.instance_id,
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

#[tracing::instrument(name = "select_server_state_from_db", skip(transaction))]
pub async fn select_server_state_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_state_id: u64,
) -> Result<ServerState, NotFoundOrUnexpectedApiError> {
    select_maybe_server_state_from_db(transaction, server_state_id)
        .await?
        .ok_or(NotFoundOrUnexpectedApiError::NotFoundError)
}

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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}

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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}

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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}

#[tracing::instrument(
    name = "select_server_states_by_server_from_db",
    skip(transaction)
)]
pub async fn select_server_states_by_server_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: String,
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
        server_id
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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}

#[tracing::instrument(
    name = "select_user_class_by_server_from_db",
    skip(transaction)
)]
pub async fn select_user_class_by_server_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: String,
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
        server_id
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

#[tracing::instrument(
    name = "select_server_states_by_server_and_project_from_db",
    skip(transaction)
)]
pub async fn select_server_states_by_server_and_project_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: String,
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
        server_id,
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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}

#[tracing::instrument(
    name = "select_server_states_by_server_and_user_from_db",
    skip(transaction)
)]
pub async fn select_server_states_by_server_and_user_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: String,
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
        server_id,
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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}

pub struct NewServerState {
    pub begin: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub instance_id: String, // UUIDv4
    pub instance_name: String,
    pub flavor: u32,
    // TODO we need an enum here
    pub status: String,
    pub user: u32,
}

// TODO really validate data
impl TryFrom<ServerStateCreateData> for NewServerState {
    type Error = String;

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
        new_server_state.instance_id,
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

#[tracing::instrument(
    name = "select_ordered_server_states_by_server_begin_and_end_from_db",
    skip(transaction)
)]
pub async fn select_ordered_server_states_by_server_begin_and_end_from_db(
    transaction: &mut Transaction<'_, MySql>,
    server_id: String,
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
                server_id
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
                server_id,
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
                server_id,
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
                server_id,
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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}

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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}

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
        .map(|r| ServerState {
            id: r.id,
            begin: r.begin.fixed_offset(),
            end: r.end.map(|end| end.fixed_offset()),
            instance_id: r.instance_id,
            instance_name: r.instance_name,
            flavor: r.flavor,
            flavor_name: r.flavor_name,
            status: r.status,
            user: r.user,
            username: r.username,
        })
        .collect::<Vec<_>>();
    Ok(rows)
}
