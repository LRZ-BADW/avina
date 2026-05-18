//! Endpoints for users and projects.

use actix_web::{
    Scope,
    web::{get, scope},
};

pub mod project;
use project::projects_scope;
#[allow(clippy::module_inception)]
pub mod user;
use user::users_scope;
pub mod me;
use me::user_me;
pub mod import;
use import::user_import;
pub mod sync;
use sync::user_sync;

/// Routes to user and project endpoints.
///
///   - `GET /api/user/projects` => [projects_scope]
///   - `GET /api/user/users` => [users_scope]
///   - `GET /api/user/me` => [user_me] endpoint
///   - `GET /api/user/import` => [user_import] endpoint
///   - `GET /api/user/sync` => [user_sync] endpoint
pub fn user_scope() -> Scope {
    scope("/user")
        .service(projects_scope())
        .service(users_scope())
        .route("/me", get().to(user_me))
        .route("/import", get().to(user_import))
        .route("/sync", get().to(user_sync))
}
