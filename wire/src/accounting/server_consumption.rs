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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConsumptionParams {
    pub begin: Option<DateTime<FixedOffset>>,
    pub end: Option<DateTime<FixedOffset>>,
    pub server: Option<Uuid>,
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
    pub detail: Option<bool>,
}
