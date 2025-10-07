use std::collections::HashMap;

use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    resources::{
        FlavorGroupUsageAggregate, FlavorGroupUsageParams,
        FlavorGroupUsageSimple, FlavorUsageSimple,
    },
    user::User,
};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};

use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
        require_user_or_project_master_or_not_found,
    },
    database::user::user::select_user_from_db,
    error::{OptionApiError, UnexpectedOnlyError},
    openstack::OpenStack,
    routes::resources::flavor::usage::{
        calculate_flavor_usage_for_project_simple,
        calculate_flavor_usage_for_user_simple,
    },
};

#[derive(Serialize)]
#[serde(untagged)]
pub enum FlavorGroupUsage {
    Simple(Vec<FlavorGroupUsageSimple>),
    Aggregate(Vec<FlavorGroupUsageAggregate>),
}

fn flavor_usage_to_flavor_group_usage(
    usages: Vec<FlavorUsageSimple>,
) -> Vec<FlavorGroupUsageSimple> {
    let mut group_usages = HashMap::new();
    for usage in usages {
        let (Some(group_id), Some(group_name)) =
            (usage.flavorgroup_id, usage.flavorgroup_name)
        else {
            continue;
        };
        let group_usages_for_user =
            group_usages.entry(usage.user_id).or_insert(HashMap::new());
        let group_usage = group_usages_for_user.entry(group_id).or_insert(
            FlavorGroupUsageSimple {
                user_id: usage.user_id,
                user_name: usage.user_name,
                flavorgroup_id: group_id,
                flavorgroup_name: group_name,
                usage: 0,
            },
        );
        group_usage.usage += usage.usage;
    }
    group_usages
        .values()
        .cloned()
        .flat_map(|h| h.values().cloned().collect::<Vec<_>>())
        .collect()
}

pub async fn calculate_flavor_group_usage_for_user_simple(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    user_id: u64,
) -> Result<Vec<FlavorGroupUsageSimple>, UnexpectedOnlyError> {
    let flavor_usage =
        calculate_flavor_usage_for_user_simple(transaction, openstack, user_id)
            .await?;
    Ok(flavor_usage_to_flavor_group_usage(flavor_usage))
}

pub async fn calculate_flavor_group_usage_for_user_aggregate(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: Data<OpenStack>,
    _user_id: u64,
) -> Result<Vec<FlavorGroupUsageAggregate>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_group_usage_for_user(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    user_id: u64,
    aggregate: bool,
) -> Result<FlavorGroupUsage, UnexpectedOnlyError> {
    Ok(if aggregate {
        FlavorGroupUsage::Aggregate(
            calculate_flavor_group_usage_for_user_aggregate(
                transaction,
                openstack,
                user_id,
            )
            .await?,
        )
    } else {
        FlavorGroupUsage::Simple(
            calculate_flavor_group_usage_for_user_simple(
                transaction,
                openstack,
                user_id,
            )
            .await?,
        )
    })
}

pub async fn calculate_flavor_group_usage_for_project_simple(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: Data<OpenStack>,
    _project_id: u64,
) -> Result<Vec<FlavorGroupUsageSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_group_usage_for_project_aggregate(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: Data<OpenStack>,
    _project_id: u64,
) -> Result<Vec<FlavorGroupUsageAggregate>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_group_usage_for_project(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    project_id: u64,
    aggregate: bool,
) -> Result<FlavorGroupUsage, UnexpectedOnlyError> {
    Ok(if aggregate {
        FlavorGroupUsage::Aggregate(
            calculate_flavor_group_usage_for_project_aggregate(
                transaction,
                openstack,
                project_id,
            )
            .await?,
        )
    } else {
        FlavorGroupUsage::Simple(
            calculate_flavor_group_usage_for_project_simple(
                transaction,
                openstack,
                project_id,
            )
            .await?,
        )
    })
}

pub async fn calculate_flavor_group_usage_for_all_simple(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: Data<OpenStack>,
) -> Result<Vec<FlavorGroupUsageSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_group_usage_for_all_aggregate(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: Data<OpenStack>,
) -> Result<Vec<FlavorGroupUsageAggregate>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_group_usage_for_all(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    aggregate: bool,
) -> Result<FlavorGroupUsage, UnexpectedOnlyError> {
    Ok(if aggregate {
        FlavorGroupUsage::Aggregate(
            calculate_flavor_group_usage_for_all_aggregate(
                transaction,
                openstack,
            )
            .await?,
        )
    } else {
        FlavorGroupUsage::Simple(
            calculate_flavor_group_usage_for_all_simple(transaction, openstack)
                .await?,
        )
    })
}

#[tracing::instrument(name = "flavor_group_usage", skip(openstack))]
pub async fn flavor_group_usage(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    openstack: Data<OpenStack>,
    params: Query<FlavorGroupUsageParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let aggregate = params.aggregate.unwrap_or(false);
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let usage = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        calculate_flavor_group_usage_for_all(
            &mut transaction,
            openstack,
            aggregate,
        )
        .await?
    } else if let Some(project_id) = params.project {
        require_master_user_or_return_not_found(&user, project_id)?;
        calculate_flavor_group_usage_for_project(
            &mut transaction,
            openstack,
            project_id.into(),
            aggregate,
        )
        .await?
    } else if let Some(user_id) = params.user {
        let user_queried =
            select_user_from_db(&mut transaction, user_id as u64).await?;
        require_user_or_project_master_or_not_found(
            &user,
            user_id,
            user_queried.project,
        )?;
        calculate_flavor_group_usage_for_user(
            &mut transaction,
            openstack,
            user_id.into(),
            aggregate,
        )
        .await?
    } else {
        calculate_flavor_group_usage_for_user(
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
