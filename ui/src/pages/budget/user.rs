use std::collections::HashMap;

use avina_wire::user::UserDetailed;
use dioxus::prelude::*;

use crate::components::charts::*;

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
