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

pub fn user_scope() -> Scope {
    scope("/user")
        .service(projects_scope())
        .service(users_scope())
        .route("/me", get().to(user_me))
        .route("/import", get().to(user_import))
        .route("/sync", get().to(user_sync))
}
