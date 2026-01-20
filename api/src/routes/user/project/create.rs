use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, ProjectCreateData, User, UserClass};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use crate::{
    authorization::require_admin_user,
    error::{MinimalApiError, NormalApiError},
};

pub struct NewProject {
    pub name: String,
    pub openstack_id: String,
    pub user_class: UserClass,
}

// TODO: validate that user class is in valid range (0-6)
impl TryFrom<ProjectCreateData> for NewProject {
    type Error = String;

    fn try_from(data: ProjectCreateData) -> Result<Self, Self::Error> {
        // TODO: really validate data, user_class range, uuid, string length
        Ok(Self {
            name: data.name,
            openstack_id: data.openstack_id,
            user_class: data.user_class.unwrap_or(UserClass::UC1),
        })
    }
}

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

#[tracing::instrument(
    name = "insert_project_into_db",
    skip(new_project, transaction)
)]
pub async fn insert_project_into_db(
    transaction: &mut Transaction<'_, MySql>,
    new_project: &NewProject,
) -> Result<u64, MinimalApiError> {
    // TODO: MariaDB 10.5 introduced INSERT ... RETURNING
    let query = sqlx::query!(
        r#"
        INSERT IGNORE INTO user_project (name, openstack_id, user_class)
        VALUES (?, ?, ?)
        "#,
        new_project.name,
        new_project.openstack_id,
        new_project.user_class as u32
    );
    let result = transaction
        .execute(query)
        .await
        .context("Failed to execute insert query")?;
    if result.rows_affected() == 0 {
        return Err(MinimalApiError::ValidationError(
            "Failed to insert new project, a conflicting entry exists"
                .to_string(),
        ));
    }
    let id = result.last_insert_id();
    Ok(id)
}
