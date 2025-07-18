use std::error::Error;

use chrono::{DateTime, FixedOffset};
use clap::Args;

#[cfg(not(feature = "user"))]
use crate::common::{find_id as user_find_id, find_id as project_find_id};
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct BudgetOverTreeFilter {
    #[clap(short, long, help = "Display entire tree", action)]
    all: bool,

    #[clap(
        short,
        long,
        help = "Display sub-tree for project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(
        short,
        long,
        help = "Display sub-tree for user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,
}

pub(crate) async fn budget_over_tree(
    api: avina::Api,
    filter: BudgetOverTreeFilter,
    end: Option<DateTime<FixedOffset>>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.budget_over_tree.get();
    if filter.all {
        request.all();
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, project).await?;
        request.project(project_id);
    } else if let Some(user) = &filter.user {
        let user_id = user_find_id(&api, user).await?;
        request.user(user_id);
    }
    if let Some(end) = end {
        request.end(end);
    }
    let result = request.send().await?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
