//! Types for avina's server consumption endpoint.

use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ServerConsumptionFlavors = HashMap<String, f64>;

pub type ServerConsumptionServer = ServerConsumptionFlavors;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ServerConsumptionUser {
    pub total: ServerConsumptionFlavors,
    pub servers: HashMap<Uuid, ServerConsumptionServer>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ServerConsumptionProject {
    pub total: ServerConsumptionFlavors,
    pub users: HashMap<String, ServerConsumptionUser>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ServerConsumptionAll {
    pub total: ServerConsumptionFlavors,
    pub projects: HashMap<String, ServerConsumptionProject>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
/// Parameters passed to the server-consumption endpoint.
///
/// Only one of the filters `server`, `user`, `project`, and `all` will actually be handled by the
/// API.
pub struct ServerConsumptionParams {
    /// Beginning of the period to calculate the consumption for (beginning of current year by default).
    pub begin: Option<DateTime<FixedOffset>>,
    /// End of the period to calculate the consumption for (end of current year by default).
    pub end: Option<DateTime<FixedOffset>>,
    /// UUID of particular server to calculate the consumption for.
    pub server: Option<Uuid>,
    /// ID of the user to calculate the consumption for.
    pub user: Option<u32>,
    /// ID of the project to calculate the consumption for.
    pub project: Option<u32>,
    /// Activate calculating the global consumption for all projects.
    pub all: Option<bool>,
    /// Activate detailed output, including complete consumption breakdown.
    pub detail: Option<bool>,
}
