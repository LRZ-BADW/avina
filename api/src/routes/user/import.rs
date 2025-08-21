use actix_web::{HttpResponse, web::ReqData};
use avina_wire::user::User;

use crate::{authorization::require_admin_user, error::NormalApiError};

#[tracing::instrument(name = "user_import")]
pub async fn user_import(
    user: ReqData<User>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    todo!()
}
