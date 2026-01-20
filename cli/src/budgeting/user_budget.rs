use std::error::Error;

use chrono::{DateTime, FixedOffset};
use clap::{Args, Subcommand};

use crate::common::{
    Execute, Format, ask_for_confirmation, print_object_list,
    print_single_object,
};
#[cfg(not(feature = "user"))]
use crate::common::{find_id as user_find_id, find_id as project_find_id};
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct UserBudgetListFilter {
    #[clap(
        short,
        long,
        help = "Display user budgets of user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Display user budgets of project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(short, long, help = "Display all user budgets", action)]
    all: bool,

    #[clap(short, long, help = "Display user budgets of the given year")]
    // TODO: validate that this is a valid year
    year: Option<u32>,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct UserBudgetOverFilter {
    #[clap(short, long, help = "Filter user budget with given ID")]
    budget: Option<u32>,

    #[clap(
        short,
        long,
        help = "Filter for user budgets of user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Filter for user budgets of project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(short, long, help = "Get information for all user budgets", action)]
    all: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum UserBudgetCommand {
    #[clap(about = "List user budgets")]
    List {
        #[clap(flatten)]
        filter: UserBudgetListFilter,
    },

    #[clap(visible_alias = "show", about = "Show user budget with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new user budget")]
    Create {
        #[clap(help = "Name, ID or OpenStack ID of the user of the budget")]
        user: String,

        #[clap(
            long,
            short,
            help = "Year of the budget, default: current year"
        )]
        year: Option<u32>,

        #[clap(long, short, help = "Amount of the budget, default: 0")]
        amount: Option<i64>,
    },

    #[clap(about = "Modify a user budget")]
    Modify {
        #[clap(help = "ID of the user budget")]
        id: u32,

        #[clap(long, short, help = "Amount of the budget")]
        amount: Option<u32>,

        #[clap(long, short, help = "Force the amount to be set", action)]
        force: bool,
    },

    #[clap(about = "Delete user budget with given ID")]
    Delete { id: u32 },

    #[clap(about = "List over status of user budgets")]
    Over {
        #[clap(flatten)]
        filter: UserBudgetOverFilter,

        #[clap(
            short,
            long,
            help = "Calculate over status up to this time [default: current time]"
        )]
        end: Option<DateTime<FixedOffset>>,

        #[clap(
            short,
            long,
            help = "Combine over status from user and project budgets",
            action
        )]
        combined: bool,

        #[clap(
            short,
            long,
            help = "Show detailed information about over status",
            action
        )]
        detail: bool,
    },

    #[clap(about = "Sync user budgets of next year to those to this one")]
    Sync,
}
pub(crate) use UserBudgetCommand::*;

impl Execute for UserBudgetCommand {
    async fn execute(
        &self,
        api: avina::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter).await,
            Get { id } => get(api, format, id).await,
            Create { user, year, amount } => {
                create(api, format, user, *year, *amount).await
            }
            Modify { id, amount, force } => {
                modify(api, format, *id, *amount, *force).await
            }
            Delete { id } => delete(api, id).await,
            Over {
                filter,
                end,
                combined,
                detail,
            } => over(api, format, filter, *end, *combined, *detail).await,
            Sync => sync(api, format).await,
        }
    }
}

async fn list(
    api: avina::Api,
    format: Format,
    filter: &UserBudgetListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user_budget.list();
    if let Some(user) = &filter.user {
        let user_id = user_find_id(&api, user).await?;
        request.user(user_id);
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, project).await?;
        request.project(project_id);
    } else if filter.all {
        request.all();
    }
    if let Some(year) = filter.year {
        request.year(year);
    }
    print_object_list(request.send().await?, format)
}

async fn get(
    api: avina::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.user_budget.get(*id).await?, format)
}

async fn create(
    api: avina::Api,
    format: Format,
    user: &str,
    year: Option<u32>,
    amount: Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let user_id = user_find_id(&api, user).await?;
    let mut request = api.user_budget.create(user_id);
    if let Some(year) = year {
        request.year(year);
    }
    if let Some(amount) = amount {
        request.amount(amount);
    }
    print_single_object(request.send().await?, format)
}

#[allow(clippy::too_many_arguments)]
async fn modify(
    api: avina::Api,
    format: Format,
    id: u32,
    amount: Option<u32>,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user_budget.modify(id);
    if let Some(amount) = amount {
        request.amount(amount);
    }
    if force {
        request.force();
    }
    print_single_object(request.send().await?, format)
}

async fn delete(api: avina::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    ask_for_confirmation()?;
    Ok(api.user_budget.delete(*id).await?)
}

async fn over(
    api: avina::Api,
    format: Format,
    filter: &UserBudgetOverFilter,
    end: Option<DateTime<FixedOffset>>,
    combined: bool,
    detail: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user_budget.over();
    if let Some(budget) = filter.budget {
        request.budget(budget);
    } else if let Some(user) = &filter.user {
        let user_id = user_find_id(&api, user).await?;
        request.user(user_id);
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, project).await?;
        request.project(project_id);
    } else if filter.all {
        request.all();
    }
    if let Some(end) = end {
        request.end(end);
    }
    match (detail, combined) {
        (false, false) => print_object_list(request.normal().await?, format),

        (true, false) => print_object_list(request.detail().await?, format),
        (false, true) => print_object_list(request.combined().await?, format),
        (true, true) => {
            print_object_list(request.combined_detail().await?, format)
        }
    }
}

async fn sync(api: avina::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.user_budget.sync().await?, format)
}
