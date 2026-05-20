//! Endpoints for flavors.

use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

pub mod create;
use create::flavor_create;
pub mod list;
use list::flavor_list;
pub mod get;
use get::flavor_get;
pub mod modify;
use modify::flavor_modify;
pub mod delete;
use delete::flavor_delete;
pub mod import;
use import::flavor_import;
pub mod usage;
use usage::flavor_usage;

/// Routes to flavor endpoints.
///
///   - `POST /api/resources/flavors/` => [flavor_create] endpoint
///   - `GET /api/resources/flavors` => [flavor_list] endpoint
///   - `GET /api/resources/flavors/{id}` => [flavor_get] endpoint
///   - `PATCH /api/resources/flavors/{id}/` => [flavor_modify] endpoint
///   - `GET /api/resources/flavors/{id}/` => [flavor_delete] endpoint
///   - `GET /api/resources/flavors/import/` => [flavor_import] endpoint
///   - `GET /api/resources/flavors/usage/` => [flavor_usage] endpoint
pub fn flavors_scope() -> Scope {
    scope("/flavors")
        .route("/", post().to(flavor_create))
        .route("", get().to(flavor_list))
        .route("/{flavor_id}", get().to(flavor_get))
        // TODO: what about PUT?
        .route("/{flavor_id}/", patch().to(flavor_modify))
        .route("/{flavor_id}/", delete().to(flavor_delete))
        .route("/import/", get().to(flavor_import))
        .route("/usage/", get().to(flavor_usage))
}

/// Wrapper type for the flavor ID parameter to user endpoints.
///
/// As this is handed to endpoints as [actix_web::web::Path], it should to have a distinguishable type.
// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
pub struct FlavorIdParam {
    /// The wrapped flavor ID.
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    flavor_id: u32,
}
