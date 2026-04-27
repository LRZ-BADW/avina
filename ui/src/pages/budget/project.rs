use std::collections::HashMap;

use avina_wire::{budgeting::BudgetOverTreeUser, user::UserDetailed};
use dioxus::prelude::*;

use crate::components::{button::*, charts::*, dialog::*};

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
    let project_budget = use_signal(|| project_tree.budget.unwrap_or(0));

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
                            total: *project_budget.read(),
                            unit: " EUR",
                            size: 100,
                        }
                    }
                    div {
                        class: "col-md-6",
                        BudgetForm {
                            prefix: user.project_name,
                            value: project_budget
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
            h3 { "Costs from Users" }
            br {}
            BarChart { caption: "Cost from individual users in EUR", data: user_cost }
        }

        br {}
        div {
            class: "row",
            h3 { "Server Cost Details" }
            br {}

            for (username, user_tree) in project_tree.users.clone() {
                div {
                    class: "col-md-3",
                    UserBudgetButtonAndDialog { username, user_tree }
                }
            }
        }
    }
}

#[component]
fn UserBudgetButtonAndDialog(
    username: String,
    user_tree: BudgetOverTreeUser,
) -> Element {
    let mut open = use_signal(|| false);

    let flavor_cost = user_tree.flavors.clone();
    let server_cost = user_tree
        .servers
        .iter()
        .map(|(k, v)| (k.to_string(), v.total))
        .collect::<HashMap<_, _>>();

    let user_budget = use_signal(|| user_tree.budget.unwrap_or(0));

    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            onclick: move |_| open.set(true),
            UsagePieChart {
                name: "{username} Budget",
                used: user_tree.cost as u64,
                total: *user_budget.read(),
                unit: " EUR",
                size: 100,
            }
        }
        DialogRoot {
            open: open(),
            on_open_change: move |v| open.set(v),
            DialogContent {
                max_width: "80%",
                max_height: "90%",
                overflow_y: "scroll",
                button {
                    class: "dialog-close",
                    r#type: "button",
                    aria_label: "Close",
                    tabindex: if open() { "0" } else { "-1" },
                    onclick: move |_| open.set(false),
                    "×"
                }
                DialogTitle { "User Budget and Cost: {username}" }
                DialogDescription {
                    div {
                        class: "container-fluid",
                        div {
                            class: "row",
                            div {
                                class: "col-md-6",
                                UsagePieChart {
                                    name: "{username} Budget",
                                    used: user_tree.cost as u64,
                                    total: *user_budget.read(),
                                    unit: " EUR",
                                    size: 100,
                                }
                            }
                            div {
                                class: "col-md-6",
                                BudgetForm {
                                    prefix: username,
                                    value: user_budget,
                                }
                            }
                        }
                        br {}
                        div {
                            class: "row",
                            h5 { "Costs from Flavors" }
                            br {}
                            BarChart { caption: "Cost from individual flavors in EUR", data: flavor_cost }
                        }
                        br {}
                        div {
                            class: "row",
                            h5 { "Costs from Servers" }
                            br {}
                            BarChart { caption: "Cost from individual servers in EUR", data: server_cost, label_size: 400 }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BudgetForm(prefix: String, mut value: Signal<u64>) -> Element {
    let mut update = use_signal(|| (*value.read()).to_string());
    let mut save = use_signal(|| false);
    let mut error = use_signal(|| None);
    if *save.read() {
        *save.write() = false;
        *error.write() = None;
        if let Ok(new_value) = (*update.read()).parse() {
            // TODO: actually try to modify the budget through the API
            *value.write() = new_value;
        } else {
            *error.write() = Some(
                "Error: Budget needs to be an integer number.".to_string(),
            );
        };
    }
    rsx! {
        div {
            class: "mb-3",
            div {
                label {
                    class: "form-label",
                    for: "{prefix}-budget-input",
                    "New budget:"
                }
                div {
                    class: "input-group",
                    span {
                        class: "input-group-text",
                        "€"
                    }
                    input {
                        type: "integer",
                        class: "form-control",
                        id: "{prefix}-budget-input",
                        aria_describedby: "{prefix}-budget-input-help",
                        value: *value.read(),
                        oninput: move |e| *update.write() = e.value(),
                    }
                    span {
                        class: "input-group-text",
                        ",00"
                    }
                }
                if let Some(error) = (*error.read()).clone() {
                    div {
                        class: "form-text",
                        b {
                            style: "color: red;",
                            "{error}"
                        }
                    }
                }
                div {
                    id: "{prefix}-budget-input-help",
                    class: "form-text",
                    "Set a new user budget here."
                }
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| *save.write() = true,
                    "Save"
                }
            }
        }
    }
}
