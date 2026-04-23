use std::collections::HashMap;

use avina_wire::user::UserDetailed;
use dioxus::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    components::{button::*, charts::*},
    pages::profile::Role,
};

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

#[component]
pub fn BudgetUserSubPage(
    api_url: String,
    token: String,
    user: UserDetailed,
) -> Element {
    let budget_over_tree = api_call!(
        api_url,
        token,
        api,
        api.budget_over_tree.get().user(user.id).send().await
    );

    let Some(project_tree) = budget_over_tree.projects.get(&user.project_name)
    else {
        return_unexpected_error!("Could not find project in budget over tree.");
    };
    let project_cost = project_tree.cost;
    let project_budget = project_tree.budget.unwrap_or(0);

    let Some(user_tree) = project_tree.users.get(&user.name) else {
        return_unexpected_error!("Could not find user in budget over tree.");
    };
    let user_cost = user_tree.cost;
    let user_budget = user_tree.budget.unwrap_or(0);

    let flavor_cost = user_tree.flavors.clone();
    let server_cost = user_tree
        .servers
        .iter()
        .map(|(k, v)| (k.to_string(), v.total))
        .collect::<HashMap<_, _>>();

    rsx! {
        div {
            class: "row",
            h3 { "Budgets" }
            br {}
            div {
                class: "col-md-6",
                div {
                    class: "row",
                    div {
                        class: "col-md-6",
                        UsagePieChart {
                            name: "Project Budget",
                            used: project_cost as u64,
                            total: project_budget,
                            unit: " EUR",
                            size: 100,
                        }
                    }
                    div {
                        class: "col-md-6",
                        UsagePieChart {
                            name: "User Budget",
                            used: user_cost as u64,
                            total: user_budget,
                            unit: " EUR",
                            size: 100,
                        }
                    }
                }
            }
            div {
                class: "col-md-6",
                h3 { "Costs from Flavors" }
                br {}
                BarChart { caption: "Cost from individual flavors in EUR", data: flavor_cost }
            }
        }

        br {}
        div {
            class: "row",
            h3 { "Costs from Servers" }
            br {}
            BarChart { caption: "Cost from individual servers in EUR", data: server_cost, label_size: 400 }
        }

        br {}
        div {
            class: "row",
            h3 { "Server Cost Details" }
            br {}
            div {
                class: "table_wrapper",
                table {
                    class: "table",
                    class: "table-striped",
                    thead {
                        tr {
                            th { "ID" },
                            th { "Total Cost [EUR]" },
                            th { "Cost from Flavors [EUR]" },
                        }
                    }
                    tbody {
                        for (uuid, server_cost) in user_tree.servers.iter() {
                            tr {
                                td { "{uuid}" }
                                td { "{server_cost.total:.2}" }
                                td {
                                    BarChart { data: server_cost.flavors.clone() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BudgetProjectSubPage(
    api_url: String,
    token: String,
    user: UserDetailed,
) -> Element {
    rsx! {
        p { "TODO" }
    }
}

#[component]
pub fn BudgetAllSubPage(
    api_url: String,
    token: String,
    user: UserDetailed,
) -> Element {
    rsx! {
        p { "TODO" }
    }
}
