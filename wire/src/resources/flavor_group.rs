use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};
use tabled::Tabled;

use crate::{resources::FlavorMinimal, user::ProjectMinimal};

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct FlavorGroup {
    pub id: u32,
    pub name: String,
    #[tabled(skip)]
    pub flavors: Vec<u32>,
    pub project: u32,
}

impl<'r> FromRow<'r, MySqlRow> for FlavorGroup {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            flavors: {
                let flavors: String = row.try_get("flavors")?;
                // TODO: can we get rid of this unwrap here
                flavors.split(',').map(|f| f.parse().unwrap()).collect()
            },
            project: row.try_get("project_id")?,
        })
    }
}

impl Display for FlavorGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, FromRow, PartialEq)]
pub struct FlavorGroupMinimal {
    pub id: u32,
    pub name: String,
}

// TODO maybe rethink the Display implementations
impl Display for FlavorGroupMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled, PartialEq)]
pub struct FlavorGroupDetailed {
    pub id: u32,
    pub name: String,
    #[tabled(skip)]
    pub flavors: Vec<FlavorMinimal>,
    pub project: ProjectMinimal,
}

impl Display for FlavorGroupDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlavorGroupListParams {
    pub all: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorGroupCreated {
    pub id: u32,
    pub name: String,
    #[tabled(skip)]
    pub flavors: Vec<FlavorMinimal>,
    pub project: u32,
}

impl Display for FlavorGroupCreated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FlavorGroup(id={}, name={})", self.id, self.name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorGroupInitialize {
    pub new_flavor_group_count: u32,
    pub new_flavor_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorGroupUsage {
    pub user_id: u32,
    pub user_name: String,
    pub flavorgroup_id: u32,
    pub flavorgroup_name: String,
    pub usage: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tabled)]
pub struct FlavorGroupUsageAggregate {
    pub flavorgroup_id: u32,
    pub flavorgroup_name: String,
    pub usage: u32,
}
