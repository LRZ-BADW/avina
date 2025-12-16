use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::{resources::FlavorGroupDetailed, user::User};
use sqlx::MySqlPool;

use super::FlavorGroupIdParam;
use crate::{
    database::{
        resources::{
            flavor::select_minimal_flavors_by_group_from_db,
            flavor_group::{
                select_flavor_group_from_db, select_lrz_flavor_group_from_db,
            },
        },
        user::project::select_project_minimal_from_db,
    },
    error::OptionApiError,
};

#[tracing::instrument(name = "flavor_group_get")]
pub async fn flavor_group_get(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorGroupIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    // TODO: this can all be condensed into one database function
    let flavor_group = if user.is_staff {
        select_flavor_group_from_db(
            &mut transaction,
            params.flavor_group_id as u64,
        )
        .await?
    } else {
        select_lrz_flavor_group_from_db(
            &mut transaction,
            params.flavor_group_id as u64,
        )
        .await?
    };
    let flavors = select_minimal_flavors_by_group_from_db(
        &mut transaction,
        params.flavor_group_id as u64,
    )
    .await?;
    let project = select_project_minimal_from_db(
        &mut transaction,
        flavor_group.project as u64,
    )
    .await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let flavor_group_detailed = FlavorGroupDetailed {
        id: flavor_group.id,
        name: flavor_group.name,
        flavors,
        project,
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_group_detailed))
}
