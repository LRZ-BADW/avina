use std::{collections::HashMap, str::FromStr};

use avina::{Api, Token, error::ApiError};
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
        api_url.clone(),
        token.clone(),
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
                            total: project_budget(),
                            unit: " EUR",
                            size: 100,
                        }
                    }
                    div {
                        class: "col-md-6",
                        BudgetForm {
                            api_url: api_url.clone(),
                            token: token.clone(),
                            prefix: user.project_name,
                            value: project_budget,
                            budget_id: project_tree.budget_id,
                            is_project_budget: Some(true),
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
            h3 { "User Budgets and Costs" }
            br {}
            p { "Click tiles to see details and set user budgets." }

            for (username, user_tree) in project_tree.users.clone() {
                div {
                    class: "col-md-3",
                    UserBudgetButtonAndDialog { api_url: api_url.clone(), token: token.clone(), username, user_tree }
                }
            }
        }
    }
}

#[component]
fn UserBudgetButtonAndDialog(
    api_url: String,
    token: String,
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
                                    total: user_budget(),
                                    unit: " EUR",
                                    size: 100,
                                }
                            }
                            div {
                                class: "col-md-6",
                                BudgetForm {
                                    api_url: api_url.clone(),
                                    token: token.clone(),
                                    prefix: username,
                                    value: user_budget,
                                    budget_id: user_tree.budget_id,
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

async fn update_budget(
    api_url: String,
    token_str: String,
    budget_id: Option<u32>,
    mut value: Signal<u64>,
    input: Signal<String>,
    mut error: Signal<Option<String>>,
    is_project_budget: bool,
) {
    let Some(budget_id) = budget_id else {
        tracing::error!("Unexpected error: no budget to configure.");
        error.set(Some("Unexpected error, please contact support.".into()));
        return;
    };
    let token = match Token::from_str(&token_str) {
        Ok(token) => token,
        Err(err) => {
            tracing::error!("{}", err);
            error.set(Some("Unexpected error, please contact support.".into()));
            return;
        }
    };
    let api = match Api::new(api_url, token, None, None) {
        Ok(api) => api,
        Err(err) => {
            tracing::error!("{}", err);
            error.set(Some("Unexpected error, please contact support.".into()));
            return;
        }
    };
    let Ok(amount) = input().parse::<u32>() else {
        error.set(Some("Budget must be a positive integer number.".into()));
        return;
    };
    if amount as u64 == value() {
        error.set(None);
        return;
    }
    let result = if is_project_budget {
        api.project_budget
            .modify(budget_id)
            .amount(amount)
            .send()
            .await
            .map(|b| b.amount)
    } else {
        api.user_budget
            .modify(budget_id)
            .amount(amount)
            .send()
            .await
            .map(|b| b.amount)
    };
    let updated_value = match result {
        Ok(value) => value,
        Err(ApiError::ResponseError(message)) => {
            tracing::warn!("API Error Response: {message}");
            error.set(Some(message.clone()));
            return;
        }
        Err(ApiError::UnexpectedError(err)) => {
            tracing::error!("Unexpected API Error: {err}");
            error.set(Some(
                "Unexpected error, please contact support.".to_string(),
            ));
            return;
        }
    };
    value.set(updated_value as u64);
    error.set(None);
}

#[component]
fn BudgetForm(
    api_url: String,
    token: String,
    budget_id: Option<u32>,
    prefix: String,
    mut value: Signal<u64>,
    is_project_budget: Option<bool>,
) -> Element {
    let mut input = use_signal(|| (*value.read()).to_string());
    let error = use_signal(|| None);

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
                        value: value(),
                        oninput: move |e| input.set(e.value()),
                    }
                    span {
                        class: "input-group-text",
                        ",00"
                    }
                }
                if let Some(error) = error().clone() {
                    div {
                        class: "form-text",
                        b {
                            style: "color: red;",
                            "Error: {error}"
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
                    onclick: move |_| {
                        let api_url = api_url.clone();
                        let token = token.clone();
                        async move {
                            update_budget(api_url, token, budget_id, value,  input, error, is_project_budget.unwrap_or(false)).await;
                        }
                    },
                    "Save"
                }
            }
        }
    }
}
