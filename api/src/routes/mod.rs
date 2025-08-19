mod accounting;
mod budgeting;
mod health_check;
mod hello;
mod pricing;
mod quota;
mod resources;
pub mod user;

pub use accounting::*;
pub use budgeting::*;
pub use health_check::*;
pub use hello::*;
pub use pricing::*;
pub use quota::*;
pub use resources::*;
pub use user::*;

// TODO: missing endpoints
// - resources::flavor::access
// - resources::flavor::usage
// - resources::flavor_group::usage
// - quota::flavor_quota::check
// - user::import

// TODO: improve the following endpoints
// - budgeting::project_budget::modify
// - budgeting::user_budget::modify
// - pricing::flavor_price::modify
// - quota::flavor_quota::modify
// - accounting::server_state::modify
// - resources::flavor_group::get
// - resources::flavor::get
// - quota::flavor_quota::get
// - budgeting::budget_over_tree
