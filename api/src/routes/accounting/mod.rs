//! Endpoints for server status, consumption and cost.

use actix_web::{Scope, web::scope};

pub mod server_state;
use server_state::server_states_scope;
pub mod server_consumption;
use server_consumption::server_consumption_scope;
pub mod server_cost;
use server_cost::server_cost_scope;

/// Main scope for routes server state, consumption and cost endpoints.
///
///   - `/api/accounting/serverstates` => [server_states_scope], see [server_state] submodule
///   - `/api/accounting/serverconsumption` => [server_consumption_scope], see
///     [server_consumption] submodule
///   - `/api/accounting/servercost` => [server_cost_scope], see [server_cost] submodule
pub fn accounting_scope() -> Scope {
    scope("/accounting")
        .service(server_states_scope())
        .service(server_consumption_scope())
        .service(server_cost_scope())
}
