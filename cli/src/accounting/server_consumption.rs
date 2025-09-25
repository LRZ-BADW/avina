use std::error::Error;

use chrono::{DateTime, FixedOffset};
use clap::Args;
use uuid::Uuid;

use crate::common::{Format, print_hashmap, print_json};
#[cfg(not(feature = "user"))]
use crate::common::{find_id as project_find_id, find_id as user_find_id};
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ServerConsumptionFilter {
    #[clap(
        short,
        long,
        help = "Calculate server consumption for server with given UUID"
    )]
    server: Option<Uuid>,

    #[clap(
        short,
        long,
        help = "Calculate server consumption for user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate server consumption for project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate server consumption for entire cloud",
        action
    )]
    all: bool,
}

pub(crate) async fn server_consumption(
    api: avina::Api,
    format: Format,
    begin: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
    filter: ServerConsumptionFilter,
    detail: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.server_consumption.get();
    if let Some(begin) = begin {
        request.begin(begin);
    }
    if let Some(end) = end {
        request.end(end);
    }
    if detail {
        if let Some(server) = filter.server {
            print_json(request.server_detail(server).await?)
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
        print_hashmap(
            if let Some(server) = filter.server {
                request.server(server).await?
            } else if let Some(user) = filter.user {
                let user_id = user_find_id(&api, &user).await?;
                request.user(user_id).await?
            } else if let Some(project) = filter.project {
                let project_id = project_find_id(&api, &project).await?;
                request.project(project_id).await?
            } else if filter.all {
                request.all().await?
            } else {
                request.mine().await?
            },
            "flavor",
            "seconds",
            format,
        )
    }
}
