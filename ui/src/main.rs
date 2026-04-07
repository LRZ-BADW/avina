use std::str::FromStr;

use avina::{Api, Token};
use dioxus::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

const API_URL: &str = "http://localhost:8000/api";

fn main() {
    launch(app);
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Page {
    Hello,
    Me,
}

macro_rules! rsx_with_page_bar {
    ($signal:ident, $content:expr) => {
        rsx! {
            div {
                for page in Page::iter() {
                    button {
                        onclick: move |_| *$signal.write() = page,
                        "{page:?}"
                    }
                }
            }
            $content
        }
    };
}

fn app() -> Element {
    let future = use_resource(move || async move {
        let mut eval = document::eval(
            r#"
            window.addEventListener("message", function(event) {
                let token = event.data;
                dioxus.send(token);
            });
            window.parent.postMessage("request-token", "*");
            "#,
        );
        let token_str: String = eval.recv().await.unwrap();
        token_str
    });
    let Some(token) = future.read_unchecked().as_ref().cloned() else {
        return rsx! { p { "Logging you in ..." } };
    };
    let mut signal = use_signal(|| Page::Hello);
    match *signal.read() {
        Page::Hello => rsx_with_page_bar!(signal, Hello { token }),
        Page::Me => rsx_with_page_bar!(signal, Me { token }),
    }
}

#[component]
fn Hello(token: String) -> Element {
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

#[component]
fn Me(token: String) -> Element {
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
