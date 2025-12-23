use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

mod create;
use create::flavor_quota_create;
mod list;
use list::flavor_quota_list;
mod get;
use get::flavor_quota_get;
mod modify;
use modify::flavor_quota_modify;
mod delete;
use delete::flavor_quota_delete;
pub mod check;
use check::flavor_quota_check;

pub fn flavor_quotas_scope() -> Scope {
    scope("/flavorquotas")
        .route("/", post().to(flavor_quota_create))
        .route("", get().to(flavor_quota_list))
        .route("/{flavor_quota_id}", get().to(flavor_quota_get))
        // TODO: what about PUT?
        .route("/{flavor_quota_id}/", patch().to(flavor_quota_modify))
        .route("/{flavor_quota_id}/", delete().to(flavor_quota_delete))
        .route("/check/", get().to(flavor_quota_check))
}

// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
struct FlavorQuotaIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    flavor_quota_id: u32,
}
