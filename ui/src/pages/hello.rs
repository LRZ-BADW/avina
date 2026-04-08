use std::str::FromStr;

use avina::{Api, Token};
use dioxus::prelude::*;

use crate::API_URL;

#[component]
pub fn HelloPage(token: String) -> Element {
    let future = use_resource(move || {
        let token_str = token.clone();
        async move {
            let token = Token::from_str(&token_str).unwrap();
            let api = Api::new(API_URL.to_string(), token, None, None).unwrap();
            api.hello.user().await.unwrap()
        }
    });
    let Some(hello) = future.read_unchecked().as_ref().cloned() else {
        return rsx! {};
    };
    rsx! {
        p { "{hello}" }
    }
}
