use actix_web::{HttpResponse, web::ReqData};
use avina_wire::user::User;

#[tracing::instrument(name = "user_import")]
pub async fn user_import(user: ReqData<User>) -> HttpResponse {
    todo!()
}
