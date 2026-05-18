//! Hello endpoints for users and admins.

use actix_web::{
    HttpResponse, Scope,
    web::{ReqData, get, scope},
};
use avina_wire::{
    hello::Hello,
    user::{Project, User},
};

use crate::{authorization::require_admin_user, error::AuthOnlyError};

/// Routes to the hello-user and hello-admin endpoints.
///
///   - `GET /api/hello` => [hello_user] endpoint
///   - `GET /api/hello/admin` => [hello_admin] endpoint
pub fn hello_scope() -> Scope {
    scope("/hello")
        .route("", get().to(hello_user))
        .route("/admin", get().to(hello_admin))
}

/// Implementation of the hello-user endpoint.
///
/// This simply returns a personalized hello message to an authenticated user.
#[tracing::instrument(name = "hello_user")]
pub async fn hello_user(
    user: ReqData<User>,
    project: ReqData<Project>,
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!(
                "Hello, {} from project {} with user class {}",
                user.name, project.name, project.user_class
            ),
        })
}

/// Implementation of the hello-admin endpoint.
///
/// This simply returns a personalized hello message to an admin user.
/// Other users will get an authorization error, when attempting to access
/// this endpoint.
#[tracing::instrument(name = "hello_admin")]
pub async fn hello_admin(
    user: ReqData<User>,
    project: ReqData<Project>,
) -> Result<HttpResponse, AuthOnlyError> {
    require_admin_user(&user)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Hello {
            message: format!(
                "Hello, admin {} from project {} with user class {}",
                user.name, project.name, project.user_class
            ),
        }))
}
