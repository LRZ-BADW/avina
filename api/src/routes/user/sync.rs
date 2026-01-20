use actix_web::{HttpResponse, web::ReqData};
use avina_wire::user::{User, UserSync};

use crate::error::NormalApiError;

#[tracing::instrument(name = "user_sync")]
pub async fn user_sync(
    user: ReqData<User>,
) -> Result<HttpResponse, NormalApiError> {
    // TODO: implement this
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(UserSync {
            updated_project_count: 0,
            updated_user_count: 0,
        }))
}
