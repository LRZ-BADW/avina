use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    resources::{FlavorUsageAggregate, FlavorUsageParams, FlavorUsageSimple},
    user::User,
};
use serde::Serialize;
use sqlx::{MySql, MySqlPool, Transaction};

use crate::{
    authorization::require_admin_user,
    error::{NormalApiError, UnexpectedOnlyError},
    openstack::OpenStack,
};

#[derive(Serialize)]
#[serde(untagged)]
pub enum FlavorUsage {
    Simple(Vec<FlavorUsageSimple>),
    Aggregate(Vec<FlavorUsageAggregate>),
}

pub async fn calculate_flavor_usage_for_user_simple(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: &OpenStack,
    _user_id: u64,
) -> Result<Vec<FlavorUsageSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_usage_for_user_aggregate(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: &OpenStack,
    _user_id: u64,
) -> Result<Vec<FlavorUsageAggregate>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_usage_for_user(
    transaction: &mut Transaction<'_, MySql>,
    openstack: &OpenStack,
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
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: &OpenStack,
    _project_id: u64,
) -> Result<Vec<FlavorUsageSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_usage_for_project_aggregate(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: &OpenStack,
    _project_id: u64,
) -> Result<Vec<FlavorUsageAggregate>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_usage_for_project(
    transaction: &mut Transaction<'_, MySql>,
    openstack: &OpenStack,
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
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: &OpenStack,
) -> Result<Vec<FlavorUsageSimple>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_usage_for_all_aggregate(
    _transaction: &mut Transaction<'_, MySql>,
    _openstack: &OpenStack,
) -> Result<Vec<FlavorUsageAggregate>, UnexpectedOnlyError> {
    todo!()
}

pub async fn calculate_flavor_usage_for_all(
    transaction: &mut Transaction<'_, MySql>,
    openstack: &OpenStack,
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
        calculate_flavor_usage_for_all(&mut transaction, &openstack, aggregate)
            .await?
    } else if let Some(project_id) = params.project {
        calculate_flavor_usage_for_project(
            &mut transaction,
            &openstack,
            project_id.into(),
            aggregate,
        )
        .await?
    } else if let Some(user_id) = params.user {
        calculate_flavor_usage_for_user(
            &mut transaction,
            &openstack,
            user_id.into(),
            aggregate,
        )
        .await?
    } else {
        calculate_flavor_usage_for_user(
            &mut transaction,
            &openstack,
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
