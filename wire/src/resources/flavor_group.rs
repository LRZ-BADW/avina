use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::{FromRow, Row, mysql::MySqlRow};
#[cfg(feature = "tabled")]
use tabled::Tabled;

use crate::{resources::FlavorMinimal, user::ProjectMinimal};

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorGroup {
    pub id: u32,
    pub name: String,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub flavors: Vec<u32>,
    pub project: u32,
}

#[cfg(feature = "sqlx")]
impl<'r> FromRow<'r, MySqlRow> for FlavorGroup {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get::<i32, _>("id")?.try_into().unwrap(),
            name: row.try_get("name")?,
            flavors: {
                let flavors: Option<String> = row.try_get("flavors")?;
                // TODO: can we get rid of this unwrap here
                match flavors {
                    Some(flavors) => flavors
                        .split(',')
                        .map(|f| f.parse::<i32>().unwrap().try_into().unwrap())
                        .collect(),
                    None => Vec::new(),
                }
            },
            project: row.try_get::<i32, _>("project")?.try_into().unwrap(),
        })
    }
}

impl Display for FlavorGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorGroupMinimal {
    pub id: u32,
    pub name: String,
}

// TODO: maybe rethink the Display implementations
impl Display for FlavorGroupMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorGroupDetailed {
    pub id: u32,
    pub name: String,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub flavors: Vec<FlavorMinimal>,
    pub project: ProjectMinimal,
}

impl Display for FlavorGroupDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct FlavorGroupListParams {
    pub all: Option<bool>,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorGroupCreated {
    pub id: u32,
    pub name: String,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub flavors: Vec<FlavorMinimal>,
    pub project: u32,
}

impl Display for FlavorGroupCreated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorGroupInitialize {
    pub new_flavor_group_count: u32,
    pub new_flavor_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlavorGroupCreateData {
    pub name: String,
    pub flavors: Vec<u32>,
}

impl FlavorGroupCreateData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            flavors: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlavorGroupModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u32>,
}

impl FlavorGroupModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            project: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct FlavorGroupUsageParams {
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
pub struct FlavorGroupUsageSimple {
    pub user_id: u32,
    pub user_name: String,
    pub flavorgroup_id: u32,
    pub flavorgroup_name: String,
    pub usage: u32,
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorGroupUsageAggregate {
    pub flavorgroup_id: u32,
    pub flavorgroup_name: String,
    pub usage: u32,
}
