use dioxus::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::{components::button::*, pages::profile::Role};

mod all;
mod project;
mod user;

use all::BudgetAllSubPage;
use project::BudgetProjectSubPage;
use user::BudgetUserSubPage;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SubPage {
    User,
    Project,
    All,
}

impl SubPage {
    fn min_role(&self) -> Role {
        match self {
            SubPage::All => Role::Admin,
            SubPage::Project => Role::Master,
            _ => Role::User,
        }
    }
}

macro_rules! rsx_with_sub_page_bar {
    ($signal:ident, $role:ident, $page:ty, $content:stmt) => {
        rsx! {
            h2 { "Budgets and Costs" }
            hr {}
            div {
                for page in SubPage::iter() {
                    if page.min_role() <= $role {
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
            }
            br {}
            div {
                class: "container-fluid",
                $content
            }
        }
    };
}

#[component]
pub fn BudgetPage(api_url: String, token: String) -> Element {
    let user =
        api_call!(api_url.clone(), token.clone(), api, api.user.me().await);
    let role = Role::from(&user);

    let mut signal = use_signal(|| SubPage::User);
    match *signal.read() {
        SubPage::User => {
            rsx_with_sub_page_bar!(
                signal,
                role,
                SubPage::User,
                BudgetUserSubPage {
                    api_url,
                    token,
                    user
                }
            )
        }
        SubPage::Project => {
            rsx_with_sub_page_bar!(
                signal,
                role,
                SubPage::Project,
                BudgetProjectSubPage {
                    api_url,
                    token,
                    user
                }
            )
        }
        SubPage::All => {
            rsx_with_sub_page_bar!(
                signal,
                role,
                SubPage::All,
                BudgetAllSubPage {
                    api_url,
                    token,
                    user
                }
            )
        }
    }
}
