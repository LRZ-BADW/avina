use std::collections::HashMap;

use dioxus::prelude::*;

use crate::components::charts::*;

#[component]
pub fn BudgetPage(api_url: String, token: String) -> Element {
    let user =
        api_call!(api_url.clone(), token.clone(), api, api.user.me().await);
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
        h2 { "Budgets and Costs" }
        hr {}

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
                BarChart { title: "Cost from individual flavors in EUR", data: flavor_cost, skip_zero: true, label_size: None }
            }
        }

        br {}
        div {
            class: "row",
            h3 { "Costs from Servers" }
            br {}
            BarChart { title: "Cost from individual servers in EUR", data: server_cost, skip_zero: true, label_size: Some(400) }
        }

    }
}
