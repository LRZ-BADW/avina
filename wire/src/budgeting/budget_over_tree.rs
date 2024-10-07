use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BudgetOverTreeServer {
    pub total: f64,
    pub flavors: HashMap<String, f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BudgetOverTreeUser {
    pub cost: f64,
    pub budget_id: u32,
    pub budget: u64,
    pub over: bool,
    pub servers: HashMap<String, BudgetOverTreeServer>,
    pub flavors: HashMap<String, f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BudgetOverTreeProject {
    pub cost: f64,
    pub budget_id: u32,
    pub budget: u64,
    pub over: bool,
    pub users: HashMap<String, BudgetOverTreeUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavors: Option<HashMap<String, f64>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BudgetOverTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    pub projects: HashMap<String, BudgetOverTreeProject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavors: Option<HashMap<String, f64>>,
}
