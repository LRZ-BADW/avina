use dioxus::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

mod components;
mod pages;

use components::button::{Button, ButtonVariant};
use pages::{HelloPage, ProfilePage};

const API_URL: &str = "http://localhost:8000/api";
const THEME_CSS: Asset = asset!("../assets/dx-components-theme.css");

fn main() {
    launch(app);
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Page {
    Profile,
    Hello,
}

macro_rules! rsx_with_page_bar {
    ($signal:ident, $page:ty, $content:stmt) => {
        rsx! {
            document::Stylesheet { href: THEME_CSS }
            div {
                for page in Page::iter() {
                    Button {
                        variant: if page == $page {
                            ButtonVariant::Ghost
                        } else {
                            ButtonVariant::Outline
                        },
                        disabled: page == $page,
                        onclick: move |_| *$signal.write() = page,
                        "{page:?}"
                    }
                }
            }
            div {
                $content
            }
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
        eval.recv::<String>().await
    });
    let token = match future.read().as_ref() {
        Some(Ok(token)) => token.clone(),
        Some(Err(error)) => {
            tracing::error!("Failed to evaluate token, due to {error}");
            return rsx! { p { b { "Error: " }, "Unexpected error, please contact support." } };
        }
        None => {
            return rsx! { p { "Logging you in ..." } };
        }
    };
    if token == "request-token" {
        tracing::error!("No token provided to UI");
        return rsx! { p { b { "Error: " }, "No token provided to UI." } };
    }
    let mut signal = use_signal(|| Page::Profile);
    match *signal.read() {
        Page::Profile => {
            rsx_with_page_bar!(signal, Page::Profile, ProfilePage { token })
        }
        Page::Hello => {
            rsx_with_page_bar!(signal, Page::Hello, HelloPage { token })
        }
    }
}
