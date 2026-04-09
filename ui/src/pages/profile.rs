use std::str::FromStr;

use avina::{Api, Token, error::ApiError};
use dioxus::prelude::*;

use crate::API_URL;

#[component]
pub fn ProfilePage(token: String) -> Element {
    let future = use_resource(move || {
        let token_str = token.clone();
        async move {
            let token = Token::from_str(&token_str)?;
            let api = Api::new(API_URL.to_string(), token, None, None)?;
            api.user.me().await
        }
    });
    let user = match future.read().as_ref() {
        Some(Ok(user)) => user.clone(),
        Some(Err(ApiError::ResponseError(message))) => {
            tracing::warn!("API Error Response: {message}");
            return rsx! { p { b { "Error: " }, "{message}" } };
        }
        Some(Err(ApiError::UnexpectedError(error))) => {
            tracing::error!("Unexpected API Error: {error}");
            return rsx! { p { b { "Error: " }, "Unexpected error, please contact support." } };
        }
        None => {
            return rsx! { p { "Loading ..." } };
        }
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
