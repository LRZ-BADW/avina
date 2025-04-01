use actix_web::{web::scope, Scope};

mod server_state;
use server_state::server_states_scope;
mod server_consumption;
use server_consumption::server_consumption_scope;
pub(crate) mod server_cost;
use server_cost::server_cost_scope;

pub fn accounting_scope() -> Scope {
    scope("/accounting")
        .service(server_states_scope())
        .service(server_consumption_scope())
        .service(server_cost_scope())
}
