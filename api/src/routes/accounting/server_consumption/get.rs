use std::collections::HashMap;

use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    accounting::{
        ServerConsumptionAll, ServerConsumptionFlavors,
        ServerConsumptionParams, ServerConsumptionProject,
        ServerConsumptionServer, ServerConsumptionUser, ServerState,
    },
    user::User,
};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};
use uuid::Uuid;

use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
        require_user_or_project_master_or_not_found,
    },
    database::{
        accounting::server_state::{
            select_ordered_server_states_by_server_begin_and_end_from_db,
            select_ordered_server_states_by_user_begin_and_end_from_db,
            select_server_states_by_server_from_db,
        },
        user::{
            project::select_all_projects_from_db,
            user::{select_user_from_db, select_users_by_project_from_db},
        },
    },
    error::{OptionApiError, UnexpectedOnlyError},
};

const CONSUMING_STATES: [&str; 15] = [
    "ACTIVE",
    "BUILD",
    "HARD_REBOOT",
    "MIGRATING",
    "PASSWORD",
    "PAUSED",
    "REBOOT",
    "REBUILD",
    "RESCUE",
    "RESIZE",
    "REVERT_RESIZE",
    "SHUTOFF",
    "SUSPENDED",
    "UNKNOWN",
    "VERIFY_RESIZE",
];

pub async fn calculate_server_consumption_for_server(
    transaction: &mut Transaction<'_, MySql>,
    server_uuid: Uuid,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    states: Option<Vec<ServerState>>,
) -> Result<ServerConsumptionServer, UnexpectedOnlyError> {
    let mut states = match states {
        Some(states) => states,
        None => {
            select_ordered_server_states_by_server_begin_and_end_from_db(
                transaction,
                server_uuid,
                begin,
                end,
            )
            .await?
        }
    };
    let mut consumption = ServerConsumptionServer::default();
    if states.is_empty() {
        return Ok(consumption);
    }
    let first = states.first_mut().unwrap();
    if let Some(begin) = begin
        && begin.fixed_offset() > first.begin
    {
        first.begin = begin.fixed_offset();
    }
    let last = states.last_mut().unwrap();
    if last.end.is_none()
        && let Some(end) = end
    {
        last.end = Some(end.fixed_offset());
    }
    if let Some(end) = end
        && end.fixed_offset() < last.end.unwrap()
    {
        last.end = Some(end.fixed_offset());
    }
    for state in states {
        let entry = consumption.entry(state.flavor_name).or_default();
        if !CONSUMING_STATES.contains(&state.status.as_str()) {
            continue;
        }
        *entry += (state.end.unwrap() - state.begin).num_seconds() as f64;
    }
    // TODO:
    Ok(consumption)
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerConsumptionForUser {
    Normal(ServerConsumptionFlavors),
    Detail(ServerConsumptionUser),
}

pub async fn calculate_server_consumption_for_user(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    detail: Option<bool>,
) -> Result<ServerConsumptionForUser, UnexpectedOnlyError> {
    let states = select_ordered_server_states_by_user_begin_and_end_from_db(
        transaction,
        user_id,
        begin,
        end,
    )
    .await?;

    let mut server_state_map: HashMap<Uuid, Vec<ServerState>> = HashMap::new();
    for state in states {
        server_state_map
            .entry(state.instance_id)
            .or_default()
            .push(state);
    }

    let mut consumption = ServerConsumptionUser::default();
    for (server_uuid, server_states) in server_state_map {
        consumption.servers.insert(
            server_uuid,
            calculate_server_consumption_for_server(
                transaction,
                server_uuid,
                begin,
                end,
                Some(server_states),
            )
            .await?,
        );
    }

    for server_consumption in consumption.servers.values() {
        for (flavor, value) in server_consumption {
            *consumption.total.entry(flavor.clone()).or_default() += value;
        }
    }

    Ok(if detail.is_some() && detail.unwrap() {
        ServerConsumptionForUser::Detail(consumption)
    } else {
        ServerConsumptionForUser::Normal(consumption.total)
    })
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerConsumptionForProject {
    Normal(ServerConsumptionFlavors),
    Detail(ServerConsumptionProject),
}

pub async fn calculate_server_consumption_for_project(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    detail: Option<bool>,
) -> Result<ServerConsumptionForProject, UnexpectedOnlyError> {
    let mut consumption = ServerConsumptionProject::default();

    let users =
        select_users_by_project_from_db(transaction, project_id).await?;
    for user in users {
        let user_consumption = match calculate_server_consumption_for_user(
            transaction,
            user.id as u64,
            begin,
            end,
            Some(true),
        )
        .await?
        {
            ServerConsumptionForUser::Normal(_normal) => unreachable!(),
            ServerConsumptionForUser::Detail(detail) => detail,
        };

        for (flavor, value) in user_consumption.total.clone() {
            *consumption.total.entry(flavor.clone()).or_default() += value;
        }

        consumption
            .users
            .insert(user.name.clone(), user_consumption);
    }

    Ok(if detail.is_some() && detail.unwrap() {
        ServerConsumptionForProject::Detail(consumption)
    } else {
        ServerConsumptionForProject::Normal(consumption.total)
    })
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerConsumptionForAll {
    Normal(ServerConsumptionFlavors),
    Detail(ServerConsumptionAll),
}

pub async fn calculate_server_consumption_for_all(
    transaction: &mut Transaction<'_, MySql>,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    detail: Option<bool>,
) -> Result<ServerConsumptionForAll, UnexpectedOnlyError> {
    let mut consumption = ServerConsumptionAll::default();

    let projects = select_all_projects_from_db(transaction).await?;
    for project in projects {
        let project_consumption =
            match calculate_server_consumption_for_project(
                transaction,
                project.id as u64,
                begin,
                end,
                Some(true),
            )
            .await?
            {
                ServerConsumptionForProject::Normal(_normal) => unreachable!(),
                ServerConsumptionForProject::Detail(detail) => detail,
            };

        for (flavor, value) in project_consumption.total.clone() {
            *consumption.total.entry(flavor.clone()).or_default() += value;
        }

        consumption
            .projects
            .insert(project.name.clone(), project_consumption);
    }

    Ok(if detail.is_some() && detail.unwrap() {
        ServerConsumptionForAll::Detail(consumption)
    } else {
        ServerConsumptionForAll::Normal(consumption.total)
    })
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ServerConsumption {
    Server(ServerConsumptionServer),
    User(ServerConsumptionForUser),
    Project(ServerConsumptionForProject),
    All(ServerConsumptionForAll),
}

#[tracing::instrument(name = "server_consumption")]
pub async fn server_consumption(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Query<ServerConsumptionParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let end = params.end.unwrap_or(Utc::now().fixed_offset());
    let begin = params.begin.unwrap_or(
        Utc.with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 0, 0)
            .unwrap()
            .fixed_offset(),
    );
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let consumption = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        ServerConsumption::All(
            calculate_server_consumption_for_all(
                &mut transaction,
                Some(begin.into()),
                Some(end.into()),
                params.detail,
            )
            .await?,
        )
    } else if let Some(project_id) = params.project {
        require_master_user_or_return_not_found(&user, project_id)?;
        ServerConsumption::Project(
            calculate_server_consumption_for_project(
                &mut transaction,
                project_id as u64,
                Some(begin.into()),
                Some(end.into()),
                params.detail,
            )
            .await?,
        )
    } else if let Some(user_id) = params.user {
        let user_queried =
            select_user_from_db(&mut transaction, user_id as u64).await?;
        require_user_or_project_master_or_not_found(
            &user,
            user_id,
            user_queried.project,
        )?;
        ServerConsumption::User(
            calculate_server_consumption_for_user(
                &mut transaction,
                user_id as u64,
                Some(begin.into()),
                Some(end.into()),
                params.detail,
            )
            .await?,
        )
    } else if let Some(server_id) = params.server {
        let server_state = select_server_states_by_server_from_db(
            &mut transaction,
            server_id,
            true,
        )
        .await?;
        let server_state_user =
            select_user_from_db(&mut transaction, server_state[0].user as u64)
                .await?;
        require_user_or_project_master_or_not_found(
            &user,
            server_state_user.id,
            server_state_user.project,
        )?;
        ServerConsumption::Server(
            calculate_server_consumption_for_server(
                &mut transaction,
                server_id,
                Some(begin.into()),
                Some(end.into()),
                None,
            )
            .await?,
        )
    } else {
        ServerConsumption::User(
            calculate_server_consumption_for_user(
                &mut transaction,
                user.id as u64,
                Some(begin.into()),
                Some(end.into()),
                params.detail,
            )
            .await?,
        )
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(consumption))
}
