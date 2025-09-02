use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use avina_wire::{resources::FlavorGroupUsageParams, user::User};
use sqlx::MySqlPool;

use crate::{error::NormalApiError, openstack::OpenStack};

#[tracing::instrument(name = "flavor_group_usage", skip(_openstack))]
pub async fn flavor_group_usage(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    _openstack: Data<OpenStack>,
    params: Query<FlavorGroupUsageParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, NormalApiError> {
    todo!()
}
