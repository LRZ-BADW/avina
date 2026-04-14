use dioxus::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

#[macro_use]
mod common;
mod components;
mod pages;

use components::button::{Button, ButtonVariant};
use pages::{PricesPage, ProfilePage, UsagePage};

// TODO: we should pass this in as arguments or so
const API_URL: &str = "https://cc.lrz.de:1338/api";
const THEME_CSS: Asset = asset!("../assets/dx-components-theme.css");

fn main() {
    launch(app);
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Page {
    Prices,
    Usage,
    Profile,
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
            br {}
            div {
                class: "container-fluid",
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
            return_unexpected_error!(
                "Failed to evaluate token, due to {}",
                error
            );
        }
        None => {
            return rsx! { p { "Logging you in ..." } };
        }
    };
    if token == "request-token" {
        tracing::error!("No token provided to UI");
        return rsx! { p { b { "Error: " }, "No token provided to UI." } };
    }
    let mut signal = use_signal(|| Page::Prices);
    let api_url = API_URL.to_string();
    match *signal.read() {
        Page::Prices => {
            rsx_with_page_bar!(
                signal,
                Page::Prices,
                PricesPage { api_url, token }
            )
        }
        Page::Usage => {
            rsx_with_page_bar!(
                signal,
                Page::Usage,
                UsagePage { api_url, token }
            )
        }
        Page::Profile => {
            rsx_with_page_bar!(
                signal,
                Page::Profile,
                ProfilePage { api_url, token }
            )
        }
    }
}
