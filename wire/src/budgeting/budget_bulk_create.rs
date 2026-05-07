//! Types for avina's budget-bulk-create endpoint.

use serde::{Deserialize, Serialize};
#[cfg(feature = "tabled")]
use tabled::Tabled;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BudgetBulkCreateData {
    pub year: i32,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BudgetBulkCreate {
    pub new_user_budget_count: u32,
    pub new_project_budget_count: u32,
}
