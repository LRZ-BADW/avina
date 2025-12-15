use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct BudgetOverTreeParams {
    pub all: Option<bool>,
    pub project: Option<u32>,
    pub user: Option<u32>,
    pub end: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct BudgetOverTreeServer {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct BudgetOverTreeUser {
    pub cost: f64,
    pub budget_id: Option<u32>,
    pub budget: Option<u64>,
    pub over: bool,
    pub servers: HashMap<Uuid, BudgetOverTreeServer>,
    pub flavors: HashMap<String, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct BudgetOverTreeProject {
    pub cost: f64,
    pub budget_id: Option<u32>,
    pub budget: Option<u64>,
    pub over: bool,
    pub users: HashMap<String, BudgetOverTreeUser>,
    // TODO: why is this an option?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavors: Option<HashMap<String, f64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct BudgetOverTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    pub projects: HashMap<String, BudgetOverTreeProject>,
    // TODO: why is this an option?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavors: Option<HashMap<String, f64>>,
}
