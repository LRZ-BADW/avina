use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::user::{User, UserCreateData};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use crate::{
    authorization::require_admin_user,
    database::user::project::select_project_from_db,
    error::{MinimalApiError, NormalApiError, OptionApiError},
};

pub struct NewUser {
    pub name: String,
    pub openstack_id: String,
    pub project_id: u32,
    pub role: u32,
    pub is_staff: bool,
    pub is_active: bool,
}

// TODO: validate that role is in range 0-2 or 3
impl TryFrom<UserCreateData> for NewUser {
    type Error = String;

    // TODO: we might need a more complex function with access to the database
    //       and the transaction
    fn try_from(data: UserCreateData) -> Result<Self, Self::Error> {
        // TODO: really validate data, role range, uuid, string length
        Ok(Self {
            name: data.name,
            openstack_id: data.openstack_id,
            project_id: data.project,
            role: data.role.unwrap_or(1),
            is_staff: data.is_staff.unwrap_or(false),
            is_active: data.is_active.unwrap_or(true),
        })
    }
}

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

#[tracing::instrument(
    name = "insert_user_into_db",
    skip(new_user, transaction)
)]
pub async fn insert_user_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_user: &NewUser,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO user_user (name, openstack_id, project_id, role, is_staff, is_active)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        new_user.name,
        new_user.openstack_id,
        new_user.project_id,
        new_user.role,
        new_user.is_staff,
        new_user.is_active,
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new user, a conflicting entry exists".to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
