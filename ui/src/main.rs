use dioxus::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

#[macro_use]
mod common;
mod components;
mod pages;

use components::button::*;
use pages::*;

const BOOTSTRAP_CSS: Asset = asset!("../assets/bootstrap.min.css");
const CHARTS_CSS: Asset = asset!("../assets/charts.min.css");
const THEME_CSS: Asset = asset!("../assets/dx-components-theme.css");

fn main() {
    launch(app);
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Page {
    Budgets,
    Prices,
    Usage,
    Profile,
}

macro_rules! rsx_with_page_bar {
    ($signal:ident, $page:ty, $content:stmt) => {
        rsx! {
            document::Stylesheet { href: BOOTSTRAP_CSS }
            document::Stylesheet { href: CHARTS_CSS }
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
            window.parent.postMessage("request", "*");
            "#,
        );
        eval.recv::<String>().await
    });
    let response = match future.read().as_ref() {
        Some(Ok(response)) => response.clone(),
        Some(Err(error)) => {
            return_unexpected_error!(
                "Failed to retrieve API URL or token, due to {}",
                error
            );
        }
        None => {
            return rsx! { p { "Logging you in ..." } };
        }
    };
    if response == "request" {
        tracing::error!("No API URL and token provided to UI.");
        return rsx! { p { b { "Error: " }, "No API URL and token provided to UI." } };
    }
    let Some((api_url, token)) = response.split_once(' ') else {
        tracing::error!("API URL and token provided to UI in invalid format.");
        return rsx! { p { b { "Error: " }, "API URL and token provided to UI in invalid format." } };
    };

    let mut signal = use_signal(|| Page::Budgets);
    match *signal.read() {
        Page::Budgets => {
            rsx_with_page_bar!(
                signal,
                Page::Budgets,
                BudgetPage { api_url, token }
            )
        }
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
