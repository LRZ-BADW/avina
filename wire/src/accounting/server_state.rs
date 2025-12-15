use std::fmt::Display;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tabled")]
use tabled::Tabled;
use uuid::Uuid;

#[cfg(feature = "tabled")]
use crate::common::display_option;

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerState {
    pub id: u32,
    pub begin: DateTime<FixedOffset>,
    #[cfg_attr(feature = "tabled", tabled(display = "display_option"))]
    pub end: Option<DateTime<FixedOffset>>,
    pub instance_id: Uuid,
    pub instance_name: String,
    pub flavor: u32,
    pub flavor_name: String,
    pub status: String,
    pub user: u32,
    pub username: String,
}

impl Display for ServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ServerState(id={})", self.id))
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerStateImport {
    pub new_state_count: u32,
    pub end_state_count: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ServerStateListParams {
    pub server: Option<Uuid>,
    pub user: Option<u32>,
    pub project: Option<u32>,
    pub all: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerStateCreateData {
    pub begin: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<FixedOffset>>,
    pub instance_id: Uuid,
    pub instance_name: String,
    pub flavor: u32,
    // TODO we need an enum here
    pub status: String,
    pub user: u32,
}

impl ServerStateCreateData {
    pub fn new(
        begin: DateTime<FixedOffset>,
        instance_id: Uuid,
        instance_name: String,
        flavor: u32,
        status: String,
        user: u32,
    ) -> Self {
        Self {
            begin,
            end: None,
            instance_id,
            instance_name,
            flavor,
            status,
            user,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerStateModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO we need an enum here
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<u32>,
}

impl ServerStateModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            begin: None,
            end: None,
            instance_id: None,
            instance_name: None,
            flavor: None,
            status: None,
            user: None,
        }
    }
}
