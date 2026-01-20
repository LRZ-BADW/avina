use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::{FromRow, Row, mysql::MySqlRow};
#[cfg(feature = "tabled")]
use tabled::Tabled;

#[cfg(feature = "tabled")]
use crate::common::display_option;
use crate::resources::FlavorGroupMinimal;

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Flavor {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub group: Option<u32>,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub group_name: Option<String>,
    pub weight: u32,
}

#[cfg(feature = "sqlx")]
impl<'r> FromRow<'r, MySqlRow> for Flavor {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        let id: u32 = row.try_get::<i32, _>("id")?.try_into().unwrap();
        let name: String = row.try_get("name")?;
        let openstack_id: String = row.try_get("openstack_id")?;
        let group: Option<u32> = row
            .try_get::<Option<i32>, _>("group_id")?
            .map(|g| g.try_into().unwrap());
        let group_name: Option<String> = row.try_get("group_name")?;
        let weight: u32 = row.try_get("weight")?;
        Ok(Flavor {
            id,
            name,
            openstack_id,
            group,
            group_name,
            weight,
        })
    }
}

impl Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Flavor(id={}, name={})", self.id, self.name))
    }
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorMinimal {
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub id: u32,
    pub name: String,
}

// TODO: maybe rethink the Display implementations
impl Display for FlavorMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Flavor(id={}, name={})", self.id, self.name))
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorDetailed {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub group: Option<FlavorGroupMinimal>,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub group_name: Option<String>,
    pub weight: u32,
}

impl Display for FlavorDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Flavor(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct FlavorListParams {
    pub all: Option<bool>,
    #[serde(rename = "flavorgroup")]
    pub group: Option<u32>,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorImport {
    pub new_flavor_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlavorCreateData {
    pub name: String,
    pub openstack_id: String, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
}

impl FlavorCreateData {
    pub fn new(name: String, openstack_id: String) -> Self {
        Self {
            name,
            openstack_id,
            group: None,
            weight: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openstack_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Option<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
}

impl FlavorModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            group: None,
            weight: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct FlavorUsageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<bool>,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorUsageSimple {
    pub user_id: u32,
    pub user_name: String,
    pub flavor_id: u32,
    pub flavor_name: String,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub flavorgroup_id: Option<u32>,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub flavorgroup_name: Option<String>,
    pub count: u32,
    pub usage: u32,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorUsageAggregate {
    pub flavor_id: u32,
    pub flavor_name: String,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub flavorgroup_id: Option<u32>,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub flavorgroup_name: Option<String>,
    pub count: u32,
    pub usage: u32,
}
