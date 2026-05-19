//! Implementation of the user-create endpoint.

use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::user::{User, UserCreateData};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::user::{
        project::select_project_from_db,
        user::{NewUser, insert_user_into_db},
    },
    error::{NormalApiError, OptionApiError},
};

/// Create a new user based on the given [UserCreateData].
///
/// On success a HTTP 201 CREATED with the created user in the response data is returned.
///
/// Only admins can call this endpoint, otherwise an [OptionApiError::AuthorizationError] is returned.
/// If the [UserCreateData] cannot be converted into a [NewUser] an [OptionApiError::ValidationError]
/// is returned.
#[tracing::instrument(name = "user_create")]
pub async fn user_create(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<UserCreateData>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    let new_user: NewUser =
        data.0.try_into().map_err(NormalApiError::ValidationError)?;
    // TODO: validate that the user exists in OpenStack
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project =
        select_project_from_db(&mut transaction, new_user.project_id as u64)
            .await?;
    let id = insert_user_into_db(&mut transaction, &new_user).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    let user_created = User {
        id: id as u32,
        name: new_user.name.clone(),
        openstack_id: new_user.openstack_id.clone(),
        project: new_user.project_id,
        project_name: project.name.clone(),
        role: new_user.role,
        is_staff: new_user.is_staff,
        is_active: new_user.is_active,
    };
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(user_created))
}
