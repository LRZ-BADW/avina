use actix_web::{
    HttpResponse,
    web::{Data, Query, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, ProjectListParams, User};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::user::project::{
        select_all_projects_from_db, select_projects_by_id_from_db,
        select_projects_by_userclass_from_db,
    },
    error::NormalApiError,
};

#[tracing::instrument(name = "project_list")]
pub async fn project_list(
    user: ReqData<User>,
    project: ReqData<Project>,
    db_pool: Data<MySqlPool>,
    params: Query<ProjectListParams>,
) -> Result<HttpResponse, NormalApiError> {
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let projects = if params.all.unwrap_or(false) {
        require_admin_user(&user)?;
        select_all_projects_from_db(&mut transaction).await?
    } else if let Some(userclass) = params.userclass {
        require_admin_user(&user)?;
        select_projects_by_userclass_from_db(&mut transaction, userclass)
            .await?
    } else {
        select_projects_by_id_from_db(&mut transaction, project.id as u64)
            .await?
    };
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(projects))
}
