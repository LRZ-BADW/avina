//! Endpoints for flavor groups.

use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

pub mod create;
use create::flavor_group_create;
pub mod list;
use list::flavor_group_list;
pub mod get;
use get::flavor_group_get;
pub mod modify;
use modify::flavor_group_modify;
pub mod delete;
use delete::flavor_group_delete;
pub mod usage;
use usage::flavor_group_usage;

/// Routes to flavor group endpoints.
///
///   - `POST /api/resources/flavorgroups/` => [flavor_group_create] endpoint
///   - `GET /api/resources/flavorgroups` => [flavor_group_list] endpoint
///   - `GET /api/resources/flavorgroups/{id}` => [flavor_group_get] endpoint
///   - `PATCH /api/resources/flavorgroups/{id}/` => [flavor_group_modify] endpoint
///   - `GET /api/resources/flavorgroups/{id}/` => [flavor_group_delete] endpoint
///   - `GET /api/resources/flavorgroups/usage/` => [flavor_group_usage] endpoint
pub fn flavor_groups_scope() -> Scope {
    scope("/flavorgroups")
        .route("/", post().to(flavor_group_create))
        .route("", get().to(flavor_group_list))
        .route("/{flavor_group_id}", get().to(flavor_group_get))
        // TODO: what about PUT?
        .route("/{flavor_group_id}/", patch().to(flavor_group_modify))
        .route("/{flavor_group_id}/", delete().to(flavor_group_delete))
        .route("/usage/", get().to(flavor_group_usage))
}

/// Wrapper type for the flavor group ID parameter to user endpoints.
///
/// As this is handed to endpoints as [actix_web::web::Path], it should to have a distinguishable type.
// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
pub struct FlavorGroupIdParam {
    /// The wrapped flavor group ID.
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    flavor_group_id: u32,
}
