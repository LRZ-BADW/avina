use std::error::Error;

use chrono::{DateTime, FixedOffset};
use clap::Args;

use crate::common::{Format, print_json, print_single_object};
#[cfg(not(feature = "user"))]
use crate::common::{find_id as project_find_id, find_id as user_find_id};
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ServerCostFilter {
    #[clap(
        short,
        long,
        help = "Calculate server cost for server with given UUID"
    )]
    // TODO validate that this is a valid server UUIDv4
    server: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate server cost for user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate server cost for project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate server cost for entire cloud",
        action
    )]
    all: bool,
}

pub(crate) async fn server_cost(
    api: avina::Api,
    format: Format,
    begin: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
    filter: ServerCostFilter,
    detail: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.server_cost.get();
    if let Some(begin) = begin {
        request.begin(begin);
    }
    if let Some(end) = end {
        request.end(end);
    }
    if detail {
        if let Some(server) = filter.server {
            print_json(request.server_detail(&server).await?)
        } else if let Some(user) = filter.user {
            let user_id = user_find_id(&api, &user).await?;
            print_json(request.user_detail(user_id).await?)
        } else if let Some(project) = filter.project {
            let project_id = project_find_id(&api, &project).await?;
            print_json(request.project_detail(project_id).await?)
        } else if filter.all {
            print_json(request.all_detail().await?)
        } else {
            print_json(request.mine_detail().await?)
        }
    } else {
        #[allow(clippy::collapsible_else_if)]
        if let Some(server) = filter.server {
            print_single_object(request.server(&server).await?, format)
        } else if let Some(user) = filter.user {
            let user_id = user_find_id(&api, &user).await?;
            print_single_object(request.user(user_id).await?, format)
        } else if let Some(project) = filter.project {
            let project_id = project_find_id(&api, &project).await?;
            print_single_object(request.project(project_id).await?, format)
        } else if filter.all {
            print_single_object(request.all().await?, format)
        } else {
            print_single_object(request.mine().await?, format)
        }
    }
}
