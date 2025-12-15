use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tabled")]
use tabled::Tabled;
use uuid::Uuid;

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ServerCostSimple {
    pub total: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ServerCostServer {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ServerCostUser {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
    pub servers: HashMap<Uuid, ServerCostServer>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ServerCostProject {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
    pub users: HashMap<String, ServerCostUser>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ServerCostAll {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
    pub projects: HashMap<String, ServerCostProject>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ServerCostParams {
    pub begin: Option<DateTime<FixedOffset>>,
    pub end: Option<DateTime<FixedOffset>>,
    pub server: Option<Uuid>,
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
    pub detail: Option<bool>,
}
