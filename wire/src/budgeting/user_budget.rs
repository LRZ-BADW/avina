use std::fmt::Display;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::FromRow;
#[cfg(feature = "tabled")]
use tabled::Tabled;

#[cfg(feature = "tabled")]
use crate::common::display_option;
use crate::common::is_false;

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBudget {
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub id: u32,
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub user: u32,
    pub username: String,
    pub year: u32,
    pub amount: u32,
}

impl Display for UserBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("UserBudget(id={})", self.id))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct UserBudgetListParams {
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
    pub year: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct UserBudgetOverParams {
    pub end: Option<DateTime<FixedOffset>>,
    pub budget: Option<u32>,
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
    pub combined: Option<bool>,
    pub detail: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBudgetCreateData {
    pub user: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

impl UserBudgetCreateData {
    pub fn new(user: u32) -> Self {
        Self {
            user,
            year: None,
            amount: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBudgetModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub force: bool,
}

impl UserBudgetModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            amount: None,
            force: false,
        }
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBudgetOverSimple {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub over: bool,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBudgetOverCombined {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub project_budget_id: Option<u32>,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBudgetOverDetail {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub over: bool,
    pub cost: f64,
    pub budget: u32,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBudgetOverCombinedDetail {
    pub budget_id: u32,
    pub user_id: u32,
    pub user_name: String,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub project_budget_id: Option<u32>,
    pub project_id: u32,
    pub project_name: String,
    pub over: bool,
    pub project_cost: f64,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub project_budget: Option<u32>,
    pub user_cost: f64,
    pub user_budget: u32,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBudgetSync {
    pub updated_budget_count: u32,
}
