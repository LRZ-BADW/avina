use std::error::Error;

use anyhow::{Context, anyhow};
use clap::{Args, Subcommand};

use crate::{
    common::{
        Execute, Format, ask_for_confirmation, print_object_list,
        print_single_object,
    },
    user::project::find_id as project_find_id,
};

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct UserListFilter {
    #[clap(short, long, help = "Display all users", action)]
    all: bool,

    #[clap(
        short,
        long,
        help = "Display users of project with given name, ID, or OpenStack ID"
    )]
    // TODO validate that this is a valid project ID
    project: Option<String>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum UserCommand {
    #[clap(about = "List users")]
    List {
        #[clap(flatten)]
        filter: UserListFilter,
    },

    #[clap(
        visible_alias = "show",
        about = "Show user with given name, ID or openstack ID"
    )]
    Get { name_or_id: String },

    #[clap(about = "Create a new user")]
    Create {
        #[clap(help = "Name of the user")]
        name: String,

        #[clap(help = "Openstack UUIDv4 of the user")]
        openstack_id: String,

        #[clap(
            help = "Name, ID or OpenStack ID of the project this user belongs to"
        )]
        project: String,

        // TODO we need some enum here
        #[clap(long, short, help = "Role of the user (1=user, 2=masteruser)")]
        role: Option<u32>,

        #[clap(long, short, help = "Whether the user is an admin", action)]
        staff: bool,

        #[clap(long, short, help = "Whether the user is inactive", action)]
        inactive: bool,
    },

    #[clap(about = "Modify a user")]
    Modify {
        #[clap(help = "Name, ID or openstack ID of the user")]
        name_or_id: String,

        #[clap(long, short, help = "Name of the user")]
        name: Option<String>,

        #[clap(long, short, help = "Openstack UUIDv4 of the user")]
        openstack_id: Option<String>,

        #[clap(
            long,
            short,
            help = "Name, ID or OpenStack ID of the project this user belongs to"
        )]
        project: Option<String>,

        // TODO we need some enum here
        #[clap(long, short, help = "Role of the user (1=user, 2=masteruser)")]
        role: Option<u32>,

        #[clap(long, short, help = "Whether the user is an admin")]
        staff: Option<bool>,

        #[clap(long, short, help = "Whether the user is inactive")]
        active: Option<bool>,
    },

    #[clap(about = "Delete user with given name, ID or openstack ID")]
    Delete { name_or_id: String },

    #[clap(about = "Show own user")]
    Me,

    #[clap(about = "Import new users and projects")]
    Import {
        #[clap(
            long,
            short,
            action,
            help = "Suppress output if nothing is imported"
        )]
        quiet: bool,
    },

    #[clap(about = "Sync users and projects to latest LDAP data")]
    Sync {
        #[clap(
            long,
            short,
            action,
            help = "Suppress output if nothing is imported"
        )]
        quiet: bool,
    },
}
pub(crate) use UserCommand::*;

impl Execute for UserCommand {
    async fn execute(
        &self,
        api: avina::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            List { filter } => list(api, format, filter).await,
            Get { name_or_id } => get(api, format, name_or_id).await,
            Create {
                name,
                openstack_id,
                project,
                role,
                staff,
                inactive,
            } => {
                create(
                    api,
                    format,
                    name.to_owned(),
                    openstack_id.to_owned(),
                    project,
                    *role,
                    *staff,
                    *inactive,
                )
                .await
            }
            Modify {
                name_or_id,
                name,
                openstack_id,
                project,
                role,
                staff,
                active,
            } => {
                modify(
                    api,
                    format,
                    name_or_id,
                    name.to_owned(),
                    openstack_id.to_owned(),
                    project.to_owned(),
                    role.to_owned(),
                    staff.to_owned(),
                    active.to_owned(),
                )
                .await
            }
            Delete { name_or_id } => delete(api, name_or_id).await,
            Me => me(api, format).await,
            Import { quiet } => import(api, format, *quiet).await,
            Sync { quiet } => sync(api, format, *quiet).await,
        }
    }
}

async fn list(
    api: avina::Api,
    format: Format,
    filter: &UserListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.user.list();
    if filter.all {
        request.all();
    } else if let Some(project) = &filter.project {
        let project_id = project_find_id(&api, project).await?;
        request.project(project_id);
    }
    print_object_list(request.send().await?, format)
}

async fn get(
    api: avina::Api,
    format: Format,
    name_or_id: &str,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id).await?;
    print_single_object(api.user.get(id).await?, format)
}

#[allow(clippy::too_many_arguments)]
async fn create(
    api: avina::Api,
    format: Format,
    name: String,
    openstack_id: String,
    project: &str,
    role: Option<u32>,
    staff: bool,
    inactive: bool,
) -> Result<(), Box<dyn Error>> {
    let project_id = project_find_id(&api, project).await?;
    let mut request = api.user.create(name, openstack_id, project_id);
    if let Some(role) = role {
        request.role(role);
    }
    if staff {
        request.staff();
    }
    if inactive {
        request.inactive();
    }
    print_single_object(request.send().await?, format)
}

#[allow(clippy::too_many_arguments)]
async fn modify(
    api: avina::Api,
    format: Format,
    name_or_id: &str,
    name: Option<String>,
    openstack_id: Option<String>,
    project: Option<String>,
    role: Option<u32>,
    staff: Option<bool>,
    active: Option<bool>,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id).await?;
    let mut request = api.user.modify(id);
    if let Some(name) = name {
        request.name(name);
    }
    if let Some(openstack_id) = openstack_id {
        request.openstack_id(openstack_id);
    }
    if let Some(project) = project {
        let project_id = project_find_id(&api, &project).await?;
        request.project(project_id);
    }
    if let Some(role) = role {
        request.role(role);
    }
    if let Some(staff) = staff {
        request.is_staff(staff);
    }
    if let Some(active) = active {
        request.is_active(active);
    }
    print_single_object(request.send().await?, format)
}

async fn delete(
    api: avina::Api,
    name_or_id: &str,
) -> Result<(), Box<dyn Error>> {
    let id = find_id(&api, name_or_id).await?;
    ask_for_confirmation()?;
    Ok(api.user.delete(id).await?)
}

async fn me(api: avina::Api, format: Format) -> Result<(), Box<dyn Error>> {
    print_single_object(api.user.me().await?, format)
}

async fn import(
    api: avina::Api,
    format: Format,
    quiet: bool,
) -> Result<(), Box<dyn Error>> {
    let result = api.user.import().await?;
    if !quiet || result.new_project_count > 0 || result.new_user_count > 0 {
        return print_single_object(result, format);
    }
    Ok(())
}

async fn sync(
    api: avina::Api,
    format: Format,
    quiet: bool,
) -> Result<(), Box<dyn Error>> {
    let result = api.user.sync().await?;
    if !quiet
        || result.updated_project_count > 0
        || result.updated_user_count > 0
    {
        return print_single_object(result, format);
    }
    Ok(())
}

pub(crate) async fn find_id(
    api: &avina::Api,
    name_or_id: &str,
) -> Result<u32, anyhow::Error> {
    if let Ok(id) = name_or_id.parse::<u32>() {
        return Ok(id);
    }
    // TODO cache me across arguments
    let me = api.user.me().await.context("Failed to get own user")?;
    let mut request = api.user.list();
    if me.is_staff {
        request.all();
    } else if me.role == 2 {
        request.project(me.project.id);
    }
    let users = request.send().await?;
    if let Some(user) = users
        .into_iter()
        .find(|u| u.openstack_id == name_or_id || u.name == name_or_id)
    {
        return Ok(user.id);
    }
    Err(anyhow!(
        "Could not find user with name or openstack ID: {name_or_id}"
    ))
}
