//! Top-level module of the avina-api.
//!
//! avina-api ships both as binary, where the main imports this, and
//! as a library, so that the unpublished `avina-test` crate may
//! import and programmatically start it as well.

pub mod authentication;
pub mod authorization;
pub mod configuration;
pub mod database;
pub mod error;
pub mod ldap;
pub mod openstack;
pub mod routes;
pub mod startup;
pub mod telemetry;
pub mod utils;
