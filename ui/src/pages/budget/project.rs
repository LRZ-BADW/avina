use std::collections::HashMap;

use avina_wire::user::UserDetailed;
use dioxus::prelude::*;

use crate::components::{
    button::{Button, ButtonVariant},
    charts::{BarChart, UsagePieChart},
};

#[component]
pub fn BudgetProjectSubPage(
    api_url: String,
    token: String,
    user: UserDetailed,
) -> Element {
    let budget_over_tree = api_call!(
        api_url,
        token,
        api,
        api.budget_over_tree
            .get()
            .project(user.project.id)
            .send()
            .await
    );

    let Some(project_tree) = budget_over_tree.projects.get(&user.project_name)
    else {
        return_unexpected_error!("Could not find project in budget over tree.");
    };
    let project_cost = project_tree.cost;
    let project_budget = project_tree.budget.unwrap_or(0);

    let Some(flavor_cost) = project_tree.flavors.clone() else {
        return_unexpected_error!(
            "Could not find project flavor cost in budget over tree."
        );
    };
    let user_cost = project_tree
        .users
        .iter()
        .map(|(k, v)| (k.to_string(), v.cost))
        .collect::<HashMap<_, _>>();

    rsx! {
        div {
            class: "row",
            h3 { "Project Budget" }
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
                        "TODO: project budget setting here"
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
            h3 { "Costs from Users" }
            br {}
            BarChart { caption: "Cost from individual users in EUR", data: user_cost }
        }

        br {}
        div {
            class: "row",
            h3 { "Server Cost Details" }
            br {}

            for (username, user_tree) in project_tree.users.iter() {
                div {
                    class: "col-md-3",
                    Button {
                        variant: ButtonVariant::Ghost,
                        UsagePieChart {
                            name: "{username} Budget",
                            used: user_tree.cost as u64,
                            total: user_tree.budget.unwrap_or(0),
                            unit: " EUR",
                            size: 100,
                        }
                    }
                }
            }
        }
    }
}
