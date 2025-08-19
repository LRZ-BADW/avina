use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::{
    budgeting::{
        BudgetOverTree, BudgetOverTreeParams, BudgetOverTreeProject,
        BudgetOverTreeServer, BudgetOverTreeUser,
    },
    user::User,
};
use chrono::{DateTime, Datelike, Utc};
use sqlx::{MySql, MySqlPool, Transaction};

use std::collections::HashMap;

use crate::{
    authorization::{
        require_admin_user, require_master_user_or_return_not_found,
        require_user_or_project_master_or_not_found,
    },
    database::{
        budgeting::{
            project_budget::{
                select_maybe_project_budget_by_project_and_year_from_db,
                select_project_budgets_by_year_from_db,
            },
            user_budget::{
                select_maybe_user_budget_by_user_and_year_from_db,
                select_user_budgets_by_project_and_year_from_db,
                select_user_budgets_by_year_from_db,
            },
        },
        user::{
            project::select_maybe_project_from_db, user::select_user_from_db,
        },
    },
    error::{
        NotFoundOrUnexpectedApiError, OptionApiError, UnexpectedOnlyError,
    },
    routes::server_cost::get::{
        calculate_server_cost_for_all_detail,
        calculate_server_cost_for_project_detail,
    },
    utils::start_of_the_year,
};

async fn budget_over_tree_for_user(
    transaction: &mut Transaction<'_, MySql>,
    user_id: u64,
    end: DateTime<Utc>,
) -> Result<BudgetOverTree, NotFoundOrUnexpectedApiError> {
    let mut tree = BudgetOverTree {
        cost: None,
        projects: HashMap::new(),
        flavors: None,
    };
    let year = end.year();
    let begin = start_of_the_year(year as u32);
    let user = select_user_from_db(transaction, user_id).await?;
    let project_budget =
        select_maybe_project_budget_by_project_and_year_from_db(
            transaction,
            user.project as u64,
            year as u32,
        )
        .await?;
    let user_budget = select_maybe_user_budget_by_user_and_year_from_db(
        transaction,
        user.id as u64,
        year as u32,
    )
    .await?;
    let project_cost = calculate_server_cost_for_project_detail(
        transaction,
        user.project as u64,
        begin,
        end,
    )
    .await?;

    tree.cost = Some(project_cost.total);
    tree.projects.insert(
        user.project_name.clone(),
        BudgetOverTreeProject {
            cost: project_cost.total,
            budget_id: None,
            budget: None,
            over: false,
            users: HashMap::new(),
            flavors: Some(project_cost.flavors),
        },
    );
    let tree_project = tree.projects.get_mut(&user.project_name).unwrap();

    if let Some(project_budget) = project_budget {
        tree_project.budget_id = Some(project_budget.id);
        tree_project.budget = Some(project_budget.amount as u64);
        tree_project.over = project_cost.total >= project_budget.amount as f64;
    }

    for (username, user_cost) in project_cost.users {
        if username != user.name {
            continue;
        }

        tree_project.users.insert(
            user.name.clone(),
            BudgetOverTreeUser {
                cost: user_cost.total,
                budget_id: None,
                budget: None,
                over: false,
                servers: HashMap::new(),
                flavors: user_cost.flavors,
            },
        );
        let tree_user = tree_project.users.get_mut(&user.name).unwrap();

        if let Some(user_budget) = user_budget {
            tree_user.budget_id = Some(user_budget.id);
            tree_user.budget = Some(user_budget.amount as u64);
            tree_user.over = user_cost.total >= user_budget.amount as f64;
        }

        for (server_uuid, server_cost) in user_cost.servers {
            tree_user.servers.insert(
                server_uuid,
                BudgetOverTreeServer {
                    total: server_cost.total,
                    flavors: server_cost.flavors,
                },
            );
        }

        break;
    }

    Ok(tree)
}

async fn budget_over_tree_for_project(
    transaction: &mut Transaction<'_, MySql>,
    project_id: u64,
    end: DateTime<Utc>,
) -> Result<BudgetOverTree, UnexpectedOnlyError> {
    let mut tree = BudgetOverTree {
        cost: None,
        projects: HashMap::new(),
        flavors: None,
    };
    let year = end.year();
    let begin = start_of_the_year(year as u32);
    let project = select_maybe_project_from_db(transaction, project_id)
        .await?
        .unwrap();
    let project_budget =
        select_maybe_project_budget_by_project_and_year_from_db(
            transaction,
            project_id,
            year as u32,
        )
        .await?;
    let user_budgets = select_user_budgets_by_project_and_year_from_db(
        transaction,
        project_id,
        year as u32,
    )
    .await?
    .iter()
    .cloned()
    .map(|b| (b.username.clone(), b))
    .collect::<HashMap<_, _>>();
    let project_cost = calculate_server_cost_for_project_detail(
        transaction,
        project_id,
        begin,
        end,
    )
    .await?;

    tree.cost = Some(project_cost.total);
    tree.projects.insert(
        project.name.clone(),
        BudgetOverTreeProject {
            cost: project_cost.total,
            budget_id: None,
            budget: None,
            over: false,
            users: HashMap::new(),
            flavors: Some(project_cost.flavors),
        },
    );
    let tree_project = tree.projects.get_mut(&project.name).unwrap();

    if let Some(project_budget) = project_budget {
        tree_project.budget_id = Some(project_budget.id);
        tree_project.budget = Some(project_budget.amount as u64);
        tree_project.over = project_cost.total >= project_budget.amount as f64;
    }

    for (username, user_cost) in project_cost.users {
        tree_project.users.insert(
            username.clone(),
            BudgetOverTreeUser {
                cost: user_cost.total,
                budget_id: None,
                budget: None,
                over: false,
                servers: HashMap::new(),
                flavors: user_cost.flavors,
            },
        );
        let tree_user = tree_project.users.get_mut(&username).unwrap();

        if let Some(user_budget) = user_budgets.get(&username) {
            tree_user.budget_id = Some(user_budget.id);
            tree_user.budget = Some(user_budget.amount as u64);
            tree_user.over = user_cost.total >= user_budget.amount as f64;
        }

        for (server_uuid, server_cost) in user_cost.servers {
            tree_user.servers.insert(
                server_uuid,
                BudgetOverTreeServer {
                    total: server_cost.total,
                    flavors: server_cost.flavors,
                },
            );
        }
    }

    Ok(tree)
}

async fn budget_over_tree_for_all(
    transaction: &mut Transaction<'_, MySql>,
    end: DateTime<Utc>,
) -> Result<BudgetOverTree, UnexpectedOnlyError> {
    let year = end.year();
    let begin = start_of_the_year(year as u32);
    let project_budgets =
        select_project_budgets_by_year_from_db(transaction, year as u32)
            .await?
            .iter()
            .cloned()
            .map(|b| (b.project_name.clone(), b))
            .collect::<HashMap<_, _>>();
    let user_budgets =
        select_user_budgets_by_year_from_db(transaction, year as u32)
            .await?
            .iter()
            .cloned()
            .map(|b| (b.username.clone(), b))
            .collect::<HashMap<_, _>>();
    let all_cost =
        calculate_server_cost_for_all_detail(transaction, begin, end).await?;
    let mut tree = BudgetOverTree {
        cost: Some(all_cost.total),
        projects: HashMap::new(),
        flavors: Some(all_cost.flavors),
    };

    for (project_name, project_cost) in all_cost.projects {
        tree.projects.insert(
            project_name.clone(),
            BudgetOverTreeProject {
                cost: project_cost.total,
                budget_id: None,
                budget: None,
                over: false,
                users: HashMap::new(),
                flavors: Some(project_cost.flavors),
            },
        );
        let tree_project = tree.projects.get_mut(&project_name).unwrap();

        if let Some(project_budget) = project_budgets.get(&project_name) {
            tree_project.budget_id = Some(project_budget.id);
            tree_project.budget = Some(project_budget.amount as u64);
            tree_project.over =
                project_cost.total >= project_budget.amount as f64;
        }

        for (username, user_cost) in project_cost.users {
            tree_project.users.insert(
                username.clone(),
                BudgetOverTreeUser {
                    cost: user_cost.total,
                    budget_id: None,
                    budget: None,
                    over: false,
                    servers: HashMap::new(),
                    flavors: user_cost.flavors,
                },
            );
            let tree_user = tree_project.users.get_mut(&username).unwrap();

            if let Some(user_budget) = user_budgets.get(&username) {
                tree_user.budget_id = Some(user_budget.id);
                tree_user.budget = Some(user_budget.amount as u64);
                tree_user.over = user_cost.total >= user_budget.amount as f64;
            }

            for (server_uuid, server_cost) in user_cost.servers {
                tree_user.servers.insert(
                    server_uuid,
                    BudgetOverTreeServer {
                        total: server_cost.total,
                        flavors: server_cost.flavors,
                    },
                );
            }
        }
    }

    Ok(tree)
}

#[tracing::instrument(name = "budget_over_tree")]
pub async fn budget_over_tree(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    params: Query<BudgetOverTreeParams>,
    // TODO: is the ValidationError variant ever used?
) -> Result<HttpResponse, OptionApiError> {
    let end = params.end.unwrap_or(Utc::now().fixed_offset());
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let over = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        budget_over_tree_for_all(&mut transaction, end.into()).await?
    } else if let Some(project_id) = params.project {
        require_master_user_or_return_not_found(&user, project_id)?;
        budget_over_tree_for_project(
            &mut transaction,
            project_id as u64,
            end.into(),
        )
        .await?
    } else if let Some(user_id) = params.user {
        let user_queried =
            select_user_from_db(&mut transaction, user_id as u64).await?;
        require_user_or_project_master_or_not_found(
            &user,
            user_id,
            user_queried.project,
        )?;
        budget_over_tree_for_user(&mut transaction, user_id as u64, end.into())
            .await?
    } else {
        budget_over_tree_for_user(&mut transaction, user.id as u64, end.into())
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(over))
}
