use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

mod create;
use create::user_budget_create;
mod list;
use list::user_budget_list;
mod get;
use get::user_budget_get;
mod modify;
use modify::user_budget_modify;
mod delete;
use delete::user_budget_delete;
mod over;
use over::user_budget_over;
mod sync;
use sync::user_budget_sync;

pub fn user_budgets_scope() -> Scope {
    scope("/userbudgets")
        .route("/", post().to(user_budget_create))
        .route("", get().to(user_budget_list))
        .route("/{user_budget_id}", get().to(user_budget_get))
        .route("/{user_budget_id}/", patch().to(user_budget_modify))
        .route("/{user_budget_id}/", delete().to(user_budget_delete))
        .route("/over/", get().to(user_budget_over))
        .route("/sync/", get().to(user_budget_sync))
}

#[derive(Deserialize, Debug)]
struct UserBudgetIdParam {
    user_budget_id: u32,
}
