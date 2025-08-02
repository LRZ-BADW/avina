use actix_web::{
    HttpResponse, Scope,
    web::{ReqData, get, scope},
};
use avina_wire::{bill::Bill, user::User};

use crate::{authorization::require_admin_user, error::NormalApiError};

pub fn bill_scope() -> Scope {
    scope("/bill").route("", get().to(bill_get))
}

#[tracing::instrument(name = "bill_get")]
async fn bill_get(user: ReqData<User>) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Bill { amount: 0.0 }))
}
