use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

pub mod create;
use create::project_create;
mod list;
use list::project_list;
pub mod get;
use get::project_get;
mod modify;
use modify::project_modify;
mod delete;
use delete::project_delete;

pub fn projects_scope() -> Scope {
    scope("/projects")
        .route("/", post().to(project_create))
        .route("", get().to(project_list))
        .route("/{project_id}", get().to(project_get))
        // TODO: what about PUT?
        .route("/{project_id}/", patch().to(project_modify))
        .route("/{project_id}/", delete().to(project_delete))
}

// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
pub struct ProjectIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    project_id: u32,
}
