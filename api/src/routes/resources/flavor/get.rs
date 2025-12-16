use actix_web::{
    HttpResponse,
    web::{Data, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::User;
use sqlx::MySqlPool;

use super::FlavorIdParam;
use crate::{
    database::resources::flavor::{
        select_flavor_detail_from_db, select_lrz_flavor_detail_from_db,
    },
    error::OptionApiError,
};

#[tracing::instrument(name = "flavor_get")]
pub async fn flavor_get(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Path<FlavorIdParam>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_detail = if user.is_staff {
        select_flavor_detail_from_db(&mut transaction, params.flavor_id as u64)
            .await?
    } else {
        select_lrz_flavor_detail_from_db(
            &mut transaction,
            params.flavor_id as u64,
        )
        .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_detail))
}
