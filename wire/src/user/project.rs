use std::fmt::Display;

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::{self, FromRow};
use strum::EnumIter;
#[cfg(feature = "tabled")]
use tabled::Tabled;

use crate::{
    error::ConversionError, resources::FlavorGroupMinimal, user::UserMinimal,
};

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Project {
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "u32"))]
    pub user_class: UserClass,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={}", self.id, self.name))
    }
}

impl PartialEq<ProjectMinimal> for Project {
    fn eq(&self, other: &ProjectMinimal) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.user_class == other.user_class
    }
}

impl PartialEq<ProjectDetailed> for Project {
    fn eq(&self, other: &ProjectDetailed) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.openstack_id == other.openstack_id
            && self.user_class == other.user_class
    }
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProjectMinimal {
    #[cfg_attr(
        feature = "sqlx",
        sqlx(try_from = "i32", rename = "project__id")
    )]
    pub id: u32,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "project__name"))]
    pub name: String,
    #[cfg_attr(
        feature = "sqlx",
        sqlx(try_from = "u32", rename = "project__user_class")
    )]
    pub user_class: UserClass,
}

impl PartialEq<Project> for ProjectMinimal {
    fn eq(&self, other: &Project) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.user_class == other.user_class
    }
}

impl PartialEq<ProjectDetailed> for ProjectMinimal {
    fn eq(&self, other: &ProjectDetailed) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.user_class == other.user_class
    }
}

impl Display for ProjectMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={})", self.id, self.name))
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProjectDetailed {
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    pub user_class: UserClass,
    // TODO rethink list output in detailed structs:
    // maybe we could have only the first few entries followed by ...
    // in the output
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub users: Vec<UserMinimal>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub flavor_groups: Vec<FlavorGroupMinimal>,
}

impl PartialEq<ProjectMinimal> for ProjectDetailed {
    fn eq(&self, other: &ProjectMinimal) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.user_class == other.user_class
    }
}

impl PartialEq<Project> for ProjectDetailed {
    fn eq(&self, other: &Project) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.openstack_id == other.openstack_id
            && self.user_class == other.user_class
    }
}

impl Display for ProjectDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Project(id={}, name={}", self.id, self.name))
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ProjectRetrieved {
    Detailed(ProjectDetailed),
    Normal(Project),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ProjectListParams {
    pub all: Option<bool>,
    pub userclass: Option<UserClass>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectCreateData {
    pub name: String,
    pub openstack_id: String, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_class: Option<UserClass>,
}

impl ProjectCreateData {
    pub fn new(name: String, openstack_id: String) -> Self {
        Self {
            name,
            openstack_id,
            user_class: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectModifyData {
    // TODO: why again is this here? since this is already a URL parameter
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openstack_id: Option<String>, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_class: Option<UserClass>,
}

impl ProjectModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            user_class: None,
        }
    }
}

#[derive(
    clap::ValueEnum,
    Hash,
    PartialEq,
    Eq,
    Clone,
    EnumIter,
    Debug,
    Deserialize,
    Serialize,
    Copy,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub enum UserClass {
    NA = 0,
    UC1 = 1,
    UC2 = 2,
    UC3 = 3,
    UC4 = 4,
    UC5 = 5,
    UC6 = 6,
}

impl Display for UserClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<u32> for UserClass {
    type Error = ConversionError;

    fn try_from(u: u32) -> Result<Self, Self::Error> {
        match u {
            0 => Ok(UserClass::NA),
            1 => Ok(UserClass::UC1),
            2 => Ok(UserClass::UC2),
            3 => Ok(UserClass::UC3),
            4 => Ok(UserClass::UC4),
            5 => Ok(UserClass::UC5),
            6 => Ok(UserClass::UC6),
            _ => Err(ConversionError(
                format!("Unknown user class value: {u}").to_string(),
            )),
        }
    }
}

impl Distribution<UserClass> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UserClass {
        match rng.random_range(0..6) {
            0 => UserClass::NA,
            1 => UserClass::UC1,
            2 => UserClass::UC2,
            3 => UserClass::UC3,
            4 => UserClass::UC4,
            5 => UserClass::UC5,
            6 => UserClass::UC6,
            _ => UserClass::NA,
        }
    }
}
