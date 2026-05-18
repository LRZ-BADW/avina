//! Endpoints for projects.

use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

pub mod create;
use create::user_create;
pub mod list;
use list::user_list;
pub mod get;
use get::user_get;
pub mod modify;
use modify::user_modify;
pub mod delete;
use delete::user_delete;

/// Routes to user endpoints.
///
///   - `POST /api/user/users/` => [user_create] endpoint
///   - `GET /api/user/users` => [user_list] endpoint
///   - `GET /api/user/users/{id}` => [user_get] endpoint
///   - `PATCH /api/user/users/{id}/` => [user_modify] endpoint
///   - `GET /api/user/users/{id}/` => [user_delete] endpoint
pub fn users_scope() -> Scope {
    scope("/users")
        .route("/", post().to(user_create))
        .route("", get().to(user_list))
        .route("/{user_id}", get().to(user_get))
        // TODO: what about PUT?
        .route("/{user_id}/", patch().to(user_modify))
        .route("/{user_id}/", delete().to(user_delete))
}

/// Wrapper type for the user ID parameter to user endpoints.
///
/// As this is handed to endpoints as [actix_web::web::Path], it should to have a distinguishable type.
// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
pub struct UserIdParam {
    /// The wrapped user ID.
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    user_id: u32,
}
