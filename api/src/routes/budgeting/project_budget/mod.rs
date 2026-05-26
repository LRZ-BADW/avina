use actix_web::{
    Scope,
    web::{delete, get, patch, post, scope},
};
use serde::Deserialize;

mod create;
use create::project_budget_create;
mod list;
use list::project_budget_list;
mod get;
use get::project_budget_get;
mod modify;
use modify::project_budget_modify;
mod delete;
use delete::project_budget_delete;
mod over;
use over::project_budget_over;

pub fn project_budgets_scope() -> Scope {
    scope("/projectbudgets")
        .route("/", post().to(project_budget_create))
        .route("", get().to(project_budget_list))
        .route("/{project_budget_id}", get().to(project_budget_get))
        // TODO: what about PUT?
        .route("/{project_budget_id}/", patch().to(project_budget_modify))
        .route("/{project_budget_id}/", delete().to(project_budget_delete))
        .route("/over/", get().to(project_budget_over))
}

#[derive(Deserialize, Debug)]
struct ProjectBudgetIdParam {
    project_budget_id: u32,
}
