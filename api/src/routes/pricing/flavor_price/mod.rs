use actix_web::web::{delete, get, patch, post, scope};
use actix_web::Scope;
use serde::Deserialize;

mod create;
use create::flavor_price_create;
// mod list;
// use list::flavor_price_list;
mod get;
use get::flavor_price_get;
mod modify;
use modify::flavor_price_modify;
mod delete;
use delete::flavor_price_delete;

pub fn flavor_prices_scope() -> Scope {
    scope("/flavorprices")
        .route("/", post().to(flavor_price_create))
        // .route("", get().to(flavor_price_list))
        .route("/{flavor_price_id}", get().to(flavor_price_get))
        // TODO: what about PUT?
        .route("/{flavor_price_id}/", patch().to(flavor_price_modify))
        .route("/{flavor_price_id}/", delete().to(flavor_price_delete))
}

// TODO: wouldn't a general IdParam be better?
#[derive(Deserialize, Debug)]
struct FlavorPriceIdParam {
    // TODO: why is this necessary, when this is clearly read in query_as
    #[allow(unused)]
    flavor_price_id: u32,
}
