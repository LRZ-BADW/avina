//! Endpoints for resources, e.g. flavors, flavor groups, and cloudusage.

use actix_web::{Scope, web::scope};

pub mod flavor_group;
use flavor_group::flavor_groups_scope;
mod flavor;
use flavor::flavors_scope;
mod usage;
use usage::usage_scope;

/// Routes to resource endpoints:
///
///   - `GET /api/resources/flavorgroups` => [flavor_groups_scope], see [flavor_group] submodule
///   - `GET /api/resources/flavors` => [flavors_scope], see [flavor] submodule
///   - `GET /api/resources/usage` => [usage_scope], e.g. the cloudusage endpoint, see [usage]
pub fn resources_scope() -> Scope {
    scope("/resources")
        .service(flavor_groups_scope())
        .service(flavors_scope())
        .service(usage_scope())
}
