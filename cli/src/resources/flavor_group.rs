use std::error::Error;

use anyhow::{Context, anyhow};
use clap::{Args, Subcommand};

use crate::common::{
    Execute, Format, ask_for_confirmation, print_object_list,
    print_single_object,
};
#[cfg(not(feature = "user"))]
use crate::common::{find_id as project_find_id, find_id as user_find_id};
#[cfg(feature = "user")]
use crate::user::{
    project::find_id as project_find_id, user::find_id as user_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct FlavorGroupListFilter {
    #[clap(short, long, help = "Display all flavors", action)]
    all: bool,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct FlavorGroupUsageFilter {
    #[clap(
        short,
        long,
        help = "Calculate flavor group usage for user with given name, ID, or OpenStack ID"
    )]
    user: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate flavor group usage for project with given name, ID, or OpenStack ID"
    )]
    project: Option<String>,

    #[clap(
        short,
        long,
        help = "Calculate flavor group usage for entire cloud",
        action
    )]
    all: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum FlavorGroupCommand {
    #[clap(about = "List flavors")]
    List {
        #[clap(flatten)]
        filter: FlavorGroupListFilter,
    },

    #[clap(
        visible_alias = "show",
        about = "Show flavor group with given name or ID"
    )]
    Get { name_or_id: String },

    #[clap(about = "Create a new flavor group")]
    Create {
        #[clap(help = "Name of the flavor group")]
        name: String,
    },

    #[clap(about = "Modify a flavor group")]
    Modify {
        #[clap(help = "Name or ID of the flavor group")]
        name_or_id: String,

        #[clap(long, short, help = "Name of the flavor group")]
        name: Option<String>,

        #[clap(
            long,
            short,
            help = "Name, ID or OpenStack ID of the project this flavor group belongs to"
        )]
        project: Option<String>,
    },

    #[clap(about = "Delete flavor group with given name or ID")]
    Delete { name_or_id: String },

    #[clap(about = "Initialize default flavor groups and flavors")]
    Initialize,

    #[clap(about = "Server cost command")]
    Usage {
        #[clap(flatten)]
        filter: FlavorGroupUsageFilter,

        #[clap(long, short = 'A', help = "Show aggregated flavor group usage")]
        aggregate: bool,
    },
}
pub(crate) use FlavorGroupCommand::*;

impl Execute for FlavorGroupCommand {
    async fn execute(
        &self,
        api: avina::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter).await,
            Get { name_or_id } => get(api, format, name_or_id).await,
            Create { name } => create(api, format, name.to_owned()).await,
            Modify {
                name_or_id,
                name,
                project,
            } => {
                modify(
                    api,
                    format,
                    name_or_id,
                    name.to_owned(),
                    project.to_owned(),
                )
                .await
            }
            Delete { name_or_id } => delete(api, name_or_id).await,
            Initialize => initialize(api, format).await,
            Usage { filter, aggregate } => {
                usage(api, format, filter, *aggregate).await
            }
        }
    }
}

async fn list(
    api: avina::Api,
    format: Format,
    filter: &FlavorGroupListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_group.list();
    if filter.all {
        request.all();
    }
    print_object_list(request.send().await?, format)
}

async fn get(
    api: avina::Api,
    format: Format,
    name_or_id: &str,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id).await?;
    print_single_object(api.flavor_group.get(id).await?, format)
}

async fn create(
    api: avina::Api,
    format: Format,
    name: String,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.flavor_group.create(name).send().await?, format)
}

async fn modify(
    api: avina::Api,
    format: Format,
    name_or_id: &str,
    name: Option<String>,
    project: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id).await?;
    let mut request = api.flavor_group.modify(id);
    if let Some(name) = name {
        request.name(name);
    }
    if let Some(project) = project {
        let project_id = project_find_id(&api, &project).await?;
        request.project(project_id);
    }
    print_single_object(request.send().await?, format)
}

async fn delete(
    api: avina::Api,
    name_or_id: &str,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id).await?;
    ask_for_confirmation()?;
    Ok(api.flavor_group.delete(id).await?)
}

async fn initialize(
    api: avina::Api,
    format: Format,
) -> Result<(), Box<dyn Error>> {
    let result = api.flavor_group.initialize().await?;
    print_single_object(result, format)
}

async fn usage(
    api: avina::Api,
    format: Format,
    filter: &FlavorGroupUsageFilter,
    aggregate: bool,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.flavor_group.usage();
    if aggregate {
        print_object_list(
            if let Some(user) = filter.user.to_owned() {
                let user_id = user_find_id(&api, &user).await?;
                request.user_aggregate(user_id).await?
            } else if let Some(project) = filter.project.to_owned() {
                let project_id = project_find_id(&api, &project).await?;
                request.project_aggregate(project_id).await?
            } else if filter.all {
                request.all_aggregate().await?
            } else {
                // TODO: this causes a http 500 error
                request.mine_aggregate().await?
            },
            format,
        )
    } else {
        print_object_list(
            if let Some(user) = filter.user.to_owned() {
                let user_id = user_find_id(&api, &user).await?;
                request.user(user_id).await?
            } else if let Some(project) = filter.project.to_owned() {
                let project_id = project_find_id(&api, &project).await?;
                request.project(project_id).await?
            } else if filter.all {
                request.all().await?
            } else {
                request.mine().await?
            },
            format,
        )
    }
}

pub(crate) async fn find_id(
    api: &avina::Api,
    name_or_id: &str,
) -> Result<u32, anyhow::Error> {
    if let Ok(id) = name_or_id.parse::<u32>() {
        return Ok(id);
    }
    // TODO: cache me across arguments
    let me = api.user.me().await.context("Failed to get own user")?;
    let mut request = api.flavor_group.list();
    if me.is_staff {
        request.all();
    }
    let projects = request.send().await?;
    if let Some(project) = projects.into_iter().find(|f| f.name == name_or_id) {
        return Ok(project.id);
    }
    Err(anyhow!(
        "Could not find flavor group with name: {name_or_id}"
    ))
}
