use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use avina_wire::{
    quota::FlavorQuotaCheckParams,
    user::{Project, User},
};
use sqlx::MySqlPool;

use crate::{authorization::require_admin_user, error::NormalApiError};

#[tracing::instrument(name = "flavor_quota_check")]
pub async fn flavor_quota_check(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    // TODO: the parameters the current scheduler filter uses are different
    params: Query<FlavorQuotaCheckParams>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    todo!()
}
