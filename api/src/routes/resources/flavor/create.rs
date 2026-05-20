//! Implementation of the flavor-create endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::{
    resources::{FlavorCreateData, FlavorDetailed, FlavorGroupMinimal},
    user::User,
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::resources::{
        flavor::insert_flavor_into_db,
        flavor_group::select_flavor_group_name_from_db,
    },
    error::OptionApiError,
};

/// Create a new flavor based on the given [FlavorCreateData].
///
/// On success a HTTP 201 CREATED with the created flavor in the response data is returned.
///
/// Only admins can call this endpoint, otherwise an [OptionApiError::AuthorizationError] is returned.
/// If the flavor group ID given in the [FlavorCreateData] does not exist, an
/// [OptionApiError::NotFoundError] is returned.
#[tracing::instrument(name = "flavor_create")]
pub async fn flavor_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let id = insert_flavor_into_db(&mut transaction, &data).await?;
    let group = if let Some(id) = data.group {
        Some(FlavorGroupMinimal {
            id,
            name: select_flavor_group_name_from_db(&mut transaction, id as u64)
                .await?,
        })
    } else {
        None
    };
    let group_name = group.as_ref().map(|g| g.name.clone());
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_created = FlavorDetailed {
        id: id as u32,
        name: data.name.clone(),
        openstack_id: data.openstack_id.clone(),
        group,
        group_name,
        weight: data.weight.unwrap_or(0),
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(flavor_created))
}
