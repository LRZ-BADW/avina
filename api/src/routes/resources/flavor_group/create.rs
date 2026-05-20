use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::{
    resources::{FlavorGroup, FlavorGroupCreateData},
    user::User,
};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::resources::flavor_group::insert_flavor_group_into_db,
    error::NormalApiError,
};

#[tracing::instrument(name = "flavor_group_create")]
pub async fn flavor_group_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorGroupCreateData>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    // TODO: the project id should be part of the FlavorGroupCreateData
    let name = data.name.clone();
    let id = insert_flavor_group_into_db(
        &mut transaction,
        &data,
        user.project as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_group_created = FlavorGroup {
        id: id as u32,
        name,
        project: user.project,
        flavors: vec![],
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(flavor_group_created))
}
