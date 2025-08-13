use actix_web::{
    Scope,
    web::{get, post, scope},
};

mod project_budget;
use project_budget::project_budgets_scope;
mod user_budget;
use user_budget::user_budgets_scope;
mod bulk_create;
use bulk_create::budget_bulk_create;
mod over_tree;
use over_tree::budget_over_tree;

pub fn budgeting_scope() -> Scope {
    scope("/budgeting")
        .service(project_budgets_scope())
        .service(user_budgets_scope())
        .route("/budgetbulkcreate/", post().to(budget_bulk_create))
        .route("/budgetovertree/", get().to(budget_over_tree))
}
