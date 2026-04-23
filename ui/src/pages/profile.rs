use std::fmt::Display;

use avina_wire::user::UserDetailed;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Unknown,
    User,
    Master,
    Admin,
}

impl From<&UserDetailed> for Role {
    fn from(value: &UserDetailed) -> Self {
        if value.is_staff {
            Role::Admin
        } else if value.role == 2 {
            Role::Master
        } else if value.role == 1 {
            Role::User
        } else {
            Role::Unknown
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => "Administrator",
            Role::Master => "Master User",
            Role::User => "User",
            Role::Unknown => "Unknown",
        }
        .fmt(f)
    }
}

#[component]
pub fn ProfilePage(api_url: String, token: String) -> Element {
    let user = api_call!(api_url, token, api, api.user.me().await);
    let role = Role::from(&user);
    rsx! {
        h2 { "Profile" }
        hr {}

        h3 { "User" }
        table {
            tr { td { b { "ID:" } }, td { "{user.id}" } }
            tr { td { b { "Name:" } }, td { "{user.name}" } }
            tr { td { b { "UUID:" } }, td { "{user.openstack_id}" } }
            tr { td { b { "Role:" } }, td { "{role}" } }
        }
        br {}

        h3 { "Project" }
        table {
            tr { td { b { "ID:" } }, td { "{user.project.id}" } }
            tr { td { b { "Name:" } }, td { "{user.project.name}" } }
            tr { td { b { "User Class:" } }, td { "{user.project.user_class}" } }
        }
        br {}

    }
}
