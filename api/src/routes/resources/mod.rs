//! Endpoints for resources, e.g. flavors, flavor groups, and cloudusage.

use actix_web::{
    Scope,
    web::{get, scope},
};

pub mod flavor_group;
use flavor_group::flavor_groups_scope;
pub mod flavor;
use flavor::flavors_scope;
pub mod usage;
use usage::cloud_usage;

/// Routes to resource endpoints:
///
///   - `/api/resources/flavorgroups` => [flavor_groups_scope], see [flavor_group] submodule
///   - `/api/resources/flavors` => [flavors_scope], see [flavor] submodule
///   - `GET /api/resources/usage` => [cloud_usage] endpoint
pub fn resources_scope() -> Scope {
    scope("/resources")
        .service(flavor_groups_scope())
        .service(flavors_scope())
        .route("/usage", get().to(cloud_usage))
}
