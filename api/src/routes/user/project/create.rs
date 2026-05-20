//! Implementation of the project-create endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, ProjectCreateData, User};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::user::project::{NewProject, insert_project_into_db},
    error::NormalApiError,
};

/// Create a new project based on the given [ProjectCreateData].
///
/// On success a HTTP 201 CREATED with the created project in the response data is returned.
///
/// Only admins can call this endpoint, otherwise an [NormalApiError::AuthorizationError] is returned.
/// If the [ProjectCreateData] cannot be converted into a [NewProject] an [NormalApiError::ValidationError]
/// is returned.
#[tracing::instrument(name = "project_create")]
pub async fn project_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectCreateData>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let new_project: NewProject =
        data.0.try_into().map_err(NormalApiError::ValidationError)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let id = insert_project_into_db(&mut transaction, &new_project).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let project_created = Project {
        id: id as u32,
        name: new_project.name.clone(),
        openstack_id: new_project.openstack_id.clone(),
        user_class: new_project.user_class,
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(project_created))
}
