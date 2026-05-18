//! Endpoints for users.

use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

pub mod create;
use create::project_create;
pub mod list;
use list::project_list;
pub mod get;
use get::project_get;
pub mod modify;
use modify::project_modify;
pub mod delete;
use delete::project_delete;

/// Routes to project endpoints.
///
///   - `POST /api/user/projects/` => [project_create] endpoint
///   - `GET /api/user/projects` => [project_list] endpoint
///   - `GET /api/user/projects/{id}` => [project_get] endpoint
///   - `PATCH /api/user/projects/{id}/` => [project_modify] endpoint
///   - `GET /api/user/projects/{id}/` => [project_delete] endpoint
pub fn projects_scope() -> Scope {
    scope("/projects")
        .route("/", post().to(project_create))
        .route("", get().to(project_list))
        .route("/{project_id}", get().to(project_get))
        // TODO: what about PUT?
        .route("/{project_id}/", patch().to(project_modify))
        .route("/{project_id}/", delete().to(project_delete))
}

/// Wrapper type for the project ID parameter to project endpoints.
///
/// As this is handed to endpoints as [actix_web::web::Path], it should to have a distinguishable type.
// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
pub struct ProjectIdParam {
    /// The wrapped project ID.
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    project_id: u32,
}
