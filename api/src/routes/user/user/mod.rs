use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

pub mod create;
use create::user_create;
mod list;
use list::user_list;
mod get;
use get::user_get;
mod modify;
use modify::user_modify;
mod delete;
use delete::user_delete;

pub fn users_scope() -> Scope {
    scope("/users")
        .route("/", post().to(user_create))
        .route("", get().to(user_list))
        .route("/{user_id}", get().to(user_get))
        // TODO: what about PUT?
        .route("/{user_id}/", patch().to(user_modify))
        .route("/{user_id}/", delete().to(user_delete))
}

// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
pub struct UserIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    user_id: u32,
}
