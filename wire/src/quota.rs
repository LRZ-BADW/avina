use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::FromRow;
#[cfg(feature = "tabled")]
use tabled::Tabled;

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorQuota {
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub id: u32,
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub user: u32,
    pub username: String,
    pub quota: i64,
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub flavor_group: u32,
    pub flavor_group_name: String,
}

impl Display for FlavorQuota {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "FlavorQuota(id={}, user={}, flavor_group={})",
            self.id, self.user, self.flavor_group
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct FlavorQuotaCheckParams {
    pub user: u32,
    pub flavor: u32,
    pub count: Option<u32>,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorQuotaCheck {
    pub underquota: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct FlavorQuotaListParams {
    pub all: Option<bool>,
    pub group: Option<u32>,
    pub user: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlavorQuotaCreateData {
    pub flavor_group: u32,
    pub user: u32,
    // TODO: maybe use Option<i64> here
    pub quota: i64,
}

impl FlavorQuotaCreateData {
    pub fn new(flavor_group: u32, user: u32) -> Self {
        Self {
            flavor_group,
            user,
            quota: -1,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlavorQuotaModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor_group: Option<u32>,
}

impl FlavorQuotaModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            user: None,
            quota: None,
            flavor_group: None,
        }
    }
}
