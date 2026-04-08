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
    rsx! {
        p { "{user:?}" }
    }
}
