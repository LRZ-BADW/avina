//! Endpoints for server states.

use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

pub mod create;
use create::server_state_create;
pub mod list;
use list::server_state_list;
pub mod get;
use get::server_state_get;
pub mod modify;
use modify::server_state_modify;
pub mod delete;
use delete::server_state_delete;
pub mod import;
use import::server_state_import;

/// Routes to server state endpoints.
///
///   - `POST /api/accounting/serverstates/` => [server_state_create] endpoint
///   - `GET /api/accounting/serverstates` => [server_state_list] endpoint
///   - `GET /api/accounting/serverstates/{id}` => [server_state_get] endpoint
///   - `PATCH /api/accounting/serverstates/{id}/` => [server_state_modify] endpoint
///   - `GET /api/accounting/serverstates/{id}/` => [server_state_delete] endpoint
///   - `GET /api/accounting/serverstates/import/` => [server_state_import] endpoint
pub fn server_states_scope() -> Scope {
    scope("/serverstates")
        .route("/", post().to(server_state_create))
        .route("", get().to(server_state_list))
        .route("/{server_state_id}", get().to(server_state_get))
        // TODO: what about PUT?
        .route("/{server_state_id}/", patch().to(server_state_modify))
        .route("/{server_state_id}/", delete().to(server_state_delete))
        .route("/import/", get().to(server_state_import))
}

/// Wrapper type for the server state ID parameter to user endpoints.
///
/// As this is handed to endpoints as [actix_web::web::Path], it should to have a distinguishable type.
// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
pub struct ServerStateIdParam {
    /// The wrapped server state ID.
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    server_state_id: u32,
}
