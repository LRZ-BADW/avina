use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    quota::{FlavorQuotaCheck, FlavorQuotaCheckParams},
    resources::Flavor,
    user::{Project, User},
};
use chrono::{DateTime, Utc};
use sqlx::{MySql, MySqlPool, Transaction};

use crate::{
    authorization::require_admin_user,
    database::{
        quota::flavor_quota::select_maybe_flavor_quota_by_user_and_group_from_db,
        resources::flavor::select_flavor_from_db,
        user::user::{
            select_user_by_openstack_id_from_db, select_user_from_db,
        },
    },
    error::{OptionApiError, UnexpectedOnlyError},
    openstack::OpenStack,
    routes::flavor_group::usage::calculate_flavor_group_usage_for_user_aggregate,
};

const CACHE_TIMEOUT_SECONDS: usize = 5;

#[derive(Hash, PartialEq, Eq)]
struct CacheKey {
    username: String,
    flavor_name: String,
    count: usize,
}

impl CacheKey {
    fn new(username: &str, flavor_name: &str, count: usize) -> Self {
        Self {
            username: username.to_string(),
            flavor_name: flavor_name.to_string(),
            count,
        }
    }
}

struct CacheValue {
    underquota: bool,
    datetime: DateTime<Utc>,
}

impl CacheValue {
    fn new(underquota: bool) -> Self {
        Self {
            underquota,
            datetime: Utc::now(),
        }
    }
}

#[derive(Default)]
pub struct QuotaCache(HashMap<CacheKey, CacheValue>);

impl QuotaCache {
    pub fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: CacheKey, underquota: bool) {
        self.0.insert(key, CacheValue::new(underquota));
    }

    fn get(&mut self, key: &CacheKey) -> Option<bool> {
        let value = self.0.get(key)?;
        if (Utc::now() - value.datetime).abs().num_seconds() as usize
            >= CACHE_TIMEOUT_SECONDS
        {
            self.0.remove(key);
            return None;
        }
        Some(value.underquota)
    }
}

async fn check_flavor_quota(
    transaction: &mut Transaction<'_, MySql>,
    openstack: Data<OpenStack>,
    user: &User,
    flavor: &Flavor,
    count: u32,
) -> Result<bool, UnexpectedOnlyError> {
    let Some(flavor_group_id) = flavor.group else {
        return Ok(false);
    };
    let Some(quota) = select_maybe_flavor_quota_by_user_and_group_from_db(
        transaction,
        user.id.into(),
        flavor_group_id.into(),
    )
    .await?
    else {
        return Ok(false);
    };
    let usage = calculate_flavor_group_usage_for_user_aggregate(
        transaction,
        openstack,
        user.id.into(),
    )
    .await?
    .iter()
    .find(|u| u.flavorgroup_id == flavor_group_id)
    .map(|u| u.usage)
    .unwrap_or(0);
    let quota = quota.quota.try_into().unwrap_or(0);
    let underquota = usage + count * flavor.weight <= quota;
    Ok(underquota)
}

#[tracing::instrument(
    name = "flavor_quota_check",
    skip(openstack, quota_cache)
)]
// TODO: the original python function cached the responses.
pub async fn flavor_quota_check(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    openstack: Data<OpenStack>,
    quota_cache: Data<Mutex<QuotaCache>>,
    params: Query<FlavorQuotaCheckParams>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let user = match (params.user, &params.openstackproject) {
        (Some(user_id), _) => {
            select_user_from_db(&mut transaction, user_id as u64).await?
        }
        (_, Some(openstack_id)) => {
            select_user_by_openstack_id_from_db(&mut transaction, openstack_id)
                .await?
        }
        _ => {
            return Err(OptionApiError::ValidationError(
                "Neither user ID nor Openstack UUID provided.".to_string(),
            ));
        }
    };
    let flavor =
        select_flavor_from_db(&mut transaction, params.flavor.into()).await?;
    let count = params.count.unwrap_or(1);
    let underquota = {
        let key = CacheKey::new(&user.name, &flavor.name, count as usize);
        let cache_result = quota_cache.lock().unwrap().get(&key);
        match cache_result {
            Some(underquota) => underquota,
            None => {
                let underquota = check_flavor_quota(
                    &mut transaction,
                    openstack,
                    &user,
                    &flavor,
                    count,
                )
                .await?;
                quota_cache.lock().unwrap().set(key, underquota);
                underquota
            }
        }
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(FlavorQuotaCheck { underquota }))
}
