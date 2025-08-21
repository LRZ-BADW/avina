use actix_web::{HttpResponse, web::ReqData};
use avina_wire::user::{User, UserImport};

use crate::{authorization::require_admin_user, error::NormalApiError};

#[tracing::instrument(name = "user_import")]
pub async fn user_import(
    user: ReqData<User>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    // TODO: implement this
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(UserImport {
            new_project_count: 0,
            new_user_count: 0,
        }))
}
