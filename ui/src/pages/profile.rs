use crate::API_URL;
use avina::{Api, Token};
use dioxus::prelude::*;
use std::str::FromStr;

#[component]
pub fn ProfilePage(token: String) -> Element {
    let future = use_resource(move || {
        let token_str = token.clone();
        async move {
            let token = Token::from_str(&token_str).unwrap();
            let api = Api::new(API_URL.to_string(), token, None, None).unwrap();
            api.user.me().await.unwrap()
        }
    });
    let Some(user) = future.read_unchecked().as_ref().cloned() else {
        return rsx! {};
    };
    let role = if user.is_staff {
        "Administrator"
    } else if user.role == 2 {
        "Master User"
    } else if user.role == 1 {
        "User"
    } else {
        "Unknown"
    };
    rsx! {
        h1 { "Profile" }
        h2 { "User" }
        table {
            tr { td { b { "ID:" } }, td { "{user.id}" } }
            tr { td { b { "Name:" } }, td { "{user.name}" } }
            tr { td { b { "UUID:" } }, td { "{user.openstack_id}" } }
            tr { td { b { "Role:" } }, td { "{role}" } }
        }
        h2 { "Project" }
        table {
            tr { td { b { "ID:" } }, td { "{user.project.id}" } }
            tr { td { b { "Name:" } }, td { "{user.project.name}" } }
            tr { td { b { "User Class:" } }, td { "{user.project.user_class}" } }
        }
    }
}
