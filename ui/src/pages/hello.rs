use std::str::FromStr;

use avina::{Api, Token, error::ApiError};
use dioxus::prelude::*;

use crate::API_URL;

#[component]
pub fn HelloPage(token: String) -> Element {
    let future = use_resource(move || {
        let token_str = token.clone();
        async move {
            let token = Token::from_str(&token_str)?;
            let api = Api::new(API_URL.to_string(), token, None, None)?;
            api.hello.user().await
        }
    });
    let hello = match future.read().as_ref() {
        Some(Ok(hello)) => hello.clone(),
        Some(Err(ApiError::ResponseError(message))) => {
            tracing::warn!("API Response Error: {message}");
            return rsx! { p { b { "Error: " }, "{message}" } };
        }
        Some(Err(ApiError::UnexpectedError(error))) => {
            tracing::error!("API Unexpected Error: {error}");
            return rsx! { p { b { "Error: " }, "Unexpected error, please contact support." } };
        }
        None => {
            return rsx! { p { "Loading ..." } };
        }
    };
    rsx! {
        p { "{hello}" }
    }
}
