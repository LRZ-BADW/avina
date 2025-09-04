use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use avina_wire::{resources::FlavorUsageParams, user::User};
use sqlx::MySqlPool;

use crate::{error::NormalApiError, openstack::OpenStack};

#[tracing::instrument(name = "flavor_usage", skip(_openstack))]
pub async fn flavor_usage(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    _openstack: Data<OpenStack>,
    params: Query<FlavorUsageParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, NormalApiError> {
    todo!()
}
