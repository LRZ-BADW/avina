use std::collections::HashMap;

use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::{Context, anyhow};
use avina_wire::{
    resources::{
        Flavor, FlavorUsageAggregate, FlavorUsageParams, FlavorUsageSimple,
    },
    user::User,
};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};

use crate::{
    authorization::require_admin_user,
    database::{
        resources::flavor::select_lrz_flavors_from_db,
        user::user::{
            select_all_users_from_db, select_maybe_user_detail_from_db,
            select_users_by_project_from_db,
        },
    },
    error::{NormalApiError, UnexpectedOnlyError},
    openstack::OpenStack,
};

#[derive(Serialize)]
#[serde(untagged)]
pub enum FlavorUsage {
    Simple(Vec<FlavorUsageSimple>),
    Aggregate(Vec<FlavorUsageAggregate>),
}

fn aggregate_flavor_usage(
    usages: Vec<FlavorUsageSimple>,
) -> Vec<FlavorUsageAggregate> {
    let mut aggregates = HashMap::new();
    for usage in usages {
        let aggregate =
            aggregates
                .entry(usage.flavor_id)
                .or_insert(FlavorUsageAggregate {
                    flavor_id: usage.flavor_id,
                    flavor_name: usage.flavor_name,
                    flavorgroup_id: usage.flavorgroup_id,
                    flavorgroup_name: usage.flavorgroup_name,
                    count: 0,
                    usage: 0,
                });
        aggregate.count += usage.count;
        aggregate.usage += usage.usage;
    }
    aggregates.values().cloned().collect()
}

pub async fn calculate_flavor_usage_for_user_simple(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    user_id: u64,
) -> Result<Vec<FlavorUsageSimple>, UnexpectedOnlyError> {
    let Some(user) =
        select_maybe_user_detail_from_db(transaction, user_id).await?
    else {
        return Err(anyhow!(format!(
            "Could not select user with id {} from database.",
            user_id
        ))
        .into());
    };
    let flavors = select_lrz_flavors_from_db(transaction).await?;
    calculate_flavor_usage_for_user_simple_inner(
        openstack,
        user.into(),
        flavors,
    )
    .await
}

pub async fn calculate_flavor_usage_for_user_simple_inner(
    openstack: Data<OpenStack>,
    user: User,
    flavors: Vec<Flavor>,
) -> Result<Vec<FlavorUsageSimple>, UnexpectedOnlyError> {
    let os_servers =
        openstack.get_servers_of_project(user.openstack_id).await?;
    let flavor_by_uuid: HashMap<_, _> = flavors
        .iter()
        .map(|f| (f.openstack_id.clone(), f.clone()))
        .collect();
    let mut flavor_usage_by_id: HashMap<_, _> = flavors
        .iter()
        .map(|flavor| {
            (
                flavor.id,
                FlavorUsageSimple {
                    user_id: user.id,
                    user_name: user.name.clone(),
                    flavor_id: flavor.id,
                    flavor_name: flavor.name.clone(),
                    flavorgroup_id: flavor.group,
                    flavorgroup_name: flavor.group_name.clone(),
                    count: 0,
                    usage: 0,
                },
            )
        })
        .collect();
    for os_server in os_servers {
        let Some(flavor) = flavor_by_uuid.get(&os_server.flavor.id) else {
            continue;
        };
        let flavor_usage =
            flavor_usage_by_id
                .entry(flavor.id)
                .or_insert(FlavorUsageSimple {
                    user_id: user.id,
                    user_name: user.name.clone(),
                    flavor_id: flavor.id,
                    flavor_name: flavor.name.clone(),
                    flavorgroup_id: flavor.group,
                    flavorgroup_name: flavor.group_name.clone(),
                    count: 0,
                    usage: 0,
                });
        flavor_usage.count += 1;
        flavor_usage.usage += flavor.weight;
    }
    Ok(flavor_usage_by_id.values().cloned().collect())
}

pub async fn calculate_flavor_usage_for_user_aggregate(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    user_id: u64,
) -> Result<Vec<FlavorUsageAggregate>, UnexpectedOnlyError> {
    Ok(aggregate_flavor_usage(
        calculate_flavor_usage_for_user_simple(transaction, openstack, user_id)
            .await?,
    ))
}

pub async fn calculate_flavor_usage_for_user(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    user_id: u64,
    aggregate: bool,
) -> Result<FlavorUsage, UnexpectedOnlyError> {
    Ok(if aggregate {
        FlavorUsage::Aggregate(
            calculate_flavor_usage_for_user_aggregate(
                transaction,
                openstack,
                user_id,
            )
            .await?,
        )
    } else {
        FlavorUsage::Simple(
            calculate_flavor_usage_for_user_simple(
                transaction,
                openstack,
                user_id,
            )
            .await?,
        )
    })
}

pub async fn calculate_flavor_usage_for_project_simple(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    project_id: u64,
) -> Result<Vec<FlavorUsageSimple>, UnexpectedOnlyError> {
    let users =
        select_users_by_project_from_db(transaction, project_id).await?;
    let flavors = select_lrz_flavors_from_db(transaction).await?;
    let mut handles = Vec::with_capacity(users.len());
    for user in users {
        handles.push(tokio::spawn(
            calculate_flavor_usage_for_user_simple_inner(
                openstack.clone(),
                user,
                flavors.clone(),
            ),
        ));
    }
    let mut usage = Vec::new();
    for handle in handles {
        usage.extend(handle.await.context("Failed to join tasks.")??)
    }
    Ok(usage)
}

pub async fn calculate_flavor_usage_for_project_aggregate(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    project_id: u64,
) -> Result<Vec<FlavorUsageAggregate>, UnexpectedOnlyError> {
    Ok(aggregate_flavor_usage(
        calculate_flavor_usage_for_project_simple(
            transaction,
            openstack,
            project_id,
        )
        .await?,
    ))
}

pub async fn calculate_flavor_usage_for_project(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    project_id: u64,
    aggregate: bool,
) -> Result<FlavorUsage, UnexpectedOnlyError> {
    Ok(if aggregate {
        FlavorUsage::Aggregate(
            calculate_flavor_usage_for_project_aggregate(
                transaction,
                openstack,
                project_id,
            )
            .await?,
        )
    } else {
        FlavorUsage::Simple(
            calculate_flavor_usage_for_project_simple(
                transaction,
                openstack,
                project_id,
            )
            .await?,
        )
    })
}

pub async fn calculate_flavor_usage_for_all_simple(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
) -> Result<Vec<FlavorUsageSimple>, UnexpectedOnlyError> {
    let users = select_all_users_from_db(transaction).await?;
    let flavors = select_lrz_flavors_from_db(transaction).await?;
    let mut handles = Vec::with_capacity(users.len());
    for user in users {
        handles.push(tokio::spawn(
            calculate_flavor_usage_for_user_simple_inner(
                openstack.clone(),
                user,
                flavors.clone(),
            ),
        ));
    }
    let mut usage = Vec::new();
    for handle in handles {
        usage.extend(handle.await.context("Failed to join tasks.")??)
    }
    Ok(usage)
}

pub async fn calculate_flavor_usage_for_all_aggregate(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
) -> Result<Vec<FlavorUsageAggregate>, UnexpectedOnlyError> {
    Ok(aggregate_flavor_usage(
        calculate_flavor_usage_for_all_simple(transaction, openstack).await?,
    ))
}

pub async fn calculate_flavor_usage_for_all(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    aggregate: bool,
) -> Result<FlavorUsage, UnexpectedOnlyError> {
    Ok(if aggregate {
        FlavorUsage::Aggregate(
            calculate_flavor_usage_for_all_aggregate(transaction, openstack)
                .await?,
        )
    } else {
        FlavorUsage::Simple(
            calculate_flavor_usage_for_all_simple(transaction, openstack)
                .await?,
        )
    })
}

#[tracing::instrument(name = "flavor_usage", skip(openstack))]
pub async fn flavor_usage(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    openstack: Data<OpenStack>,
    params: Query<FlavorUsageParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let aggregate = params.aggregate.unwrap_or(false);
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let usage = if params.all.unwrap_or(false) {
        calculate_flavor_usage_for_all(&mut transaction, openstack, aggregate)
            .await?
    } else if let Some(project_id) = params.project {
        calculate_flavor_usage_for_project(
            &mut transaction,
            openstack,
            project_id.into(),
            aggregate,
        )
        .await?
    } else if let Some(user_id) = params.user {
        calculate_flavor_usage_for_user(
            &mut transaction,
            openstack,
            user_id.into(),
            aggregate,
        )
        .await?
    } else {
        calculate_flavor_usage_for_user(
            &mut transaction,
            openstack,
            user.id.into(),
            aggregate,
        )
        .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(usage))
}
