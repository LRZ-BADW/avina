use std::{cmp::PartialEq, fmt::Display};

use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::FromRow;
#[cfg(feature = "tabled")]
use tabled::Tabled;

use crate::user::ProjectMinimal;

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct User {
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub project: u32,
    pub project_name: String,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

impl PartialEq<UserMinimal> for User {
    fn eq(&self, other: &UserMinimal) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl PartialEq<UserDetailed> for User {
    fn eq(&self, other: &UserDetailed) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.openstack_id == other.openstack_id
            && self.project == other.project.id
            && self.project_name == other.project_name
            && self.project_name == other.project.name
            && self.is_staff == other.is_staff
            && self.is_active == other.is_active
            && self.role == other.role
    }
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserMinimal {
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub id: u32,
    pub name: String,
}

impl PartialEq<User> for UserMinimal {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl PartialEq<UserDetailed> for UserMinimal {
    fn eq(&self, other: &UserDetailed) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Display for UserMinimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User(id={}, name={}", self.id, self.name))
    }
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserDetailed {
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i32"))]
    pub id: u32,
    pub name: String,
    pub openstack_id: String, // UUIDv4 without dashes
    #[cfg_attr(feature = "sqlx", sqlx(flatten))]
    pub project: ProjectMinimal,
    pub project_name: String,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

impl PartialEq<UserMinimal> for UserDetailed {
    fn eq(&self, other: &UserMinimal) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl PartialEq<User> for UserDetailed {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.openstack_id == other.openstack_id
            && self.project.id == other.project
            && self.project.name == other.project_name
            && self.project_name == other.project_name
            && self.is_staff == other.is_staff
            && self.is_active == other.is_active
            && self.role == other.role
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserImport {
    pub new_project_count: u32,
    pub new_user_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListParams {
    pub all: Option<bool>,
    pub project: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserCreateData {
    pub name: String,
    pub openstack_id: String, // UUIDv4
    // TODO can't this be optional?
    pub project: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    // this could be an enum right
    pub role: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl UserCreateData {
    pub fn new(name: String, openstack_id: String, project: u32) -> Self {
        Self {
            name,
            openstack_id,
            project,
            role: None,
            is_staff: None,
            is_active: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserModifyData {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openstack_id: Option<String>, // UUIDv4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl UserModifyData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            openstack_id: None,
            project: None,
            role: None,
            is_staff: None,
            is_active: None,
        }
    }
}
