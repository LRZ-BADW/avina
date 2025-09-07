use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::user::{Project, ProjectModifyData, User};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::ProjectIdParam;
use crate::{
    authorization::require_admin_user,
    database::user::project::select_project_from_db,
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
};

#[tracing::instrument(name = "project_modify")]
pub async fn project_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<ProjectModifyData>,
    params: Path<ProjectIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.project_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let project = update_project_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(project))
}

#[tracing::instrument(name = "update_project_in_db", skip(data, transaction))]
pub async fn update_project_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &ProjectModifyData,
) -> Result<Project, NotFoundOrUnexpectedApiError> {
    let row = select_project_from_db(transaction, data.id as u64).await?;
    let name = data.name.clone().unwrap_or(row.name);
    let openstack_id = data.openstack_id.clone().unwrap_or(row.openstack_id);
    let user_class = data.user_class.unwrap_or(row.user_class);
    let query = sqlx::query!(
        r#"
        UPDATE user_project
        SET name = ?, openstack_id = ?, user_class = ?
        WHERE id = ?
        "#,
        name,
        openstack_id,
        user_class as u32,
        data.id,
    );
    transaction
        .execute(query)
        .await
        .context("Failed to execute update query")?;
    let project = Project {
        id: data.id,
        name,
        openstack_id,
        user_class,
    };
    Ok(project)
}
