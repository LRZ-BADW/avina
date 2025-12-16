use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, ReqData},
};
use anyhow::Context;
use avina_wire::{
    quota::{FlavorQuota, FlavorQuotaModifyData},
    user::User,
};
use sqlx::{Executor, MySql, MySqlPool, Transaction};

use super::FlavorQuotaIdParam;
use crate::{
    authorization::require_admin_user,
    database::{
        quota::flavor_quota::select_flavor_quota_from_db,
        resources::flavor_group::select_flavor_group_name_from_db,
        user::user::select_user_name_from_db,
    },
    error::{NotFoundOrUnexpectedApiError, OptionApiError},
};

#[tracing::instrument(name = "flavor_quota_modify")]
pub async fn flavor_quota_modify(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    data: Json<FlavorQuotaModifyData>,
    params: Path<FlavorQuotaIdParam>,
) -> Result<HttpResponse, OptionApiError> {
    require_admin_user(&user)?;
    // TODO: do further validation
    if data.id != params.flavor_quota_id {
        return Err(OptionApiError::ValidationError(
            "ID in URL does not match ID in body".to_string(),
        ));
    }
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;
    let flavor_quota =
        update_flavor_quota_in_db(&mut transaction, &data).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(flavor_quota))
}

#[tracing::instrument(
    name = "update_flavor_quota_in_db",
    skip(data, transaction)
)]
pub async fn update_flavor_quota_in_db(
    transaction: &mut Transaction<'_, MySql>,
    data: &FlavorQuotaModifyData,
) -> Result<FlavorQuota, NotFoundOrUnexpectedApiError> {
    let row = select_flavor_quota_from_db(transaction, data.id as u64).await?;
    let user = data.user.unwrap_or(row.user);
    let username = select_user_name_from_db(transaction, user as u64).await?;
    let quota = data.quota.unwrap_or(row.quota);
    let flavor_group = data.flavor_group.unwrap_or(row.flavor_group);
    let flavor_group_name =
        select_flavor_group_name_from_db(transaction, flavor_group as u64)
            .await?;
    let query1 = sqlx::query!(
        r#"
        UPDATE quota_quota
        SET
            user_id = ?,
            quota = ?
        WHERE id = ?
        "#,
        user,
        quota,
        data.id,
    );
    transaction
        .execute(query1)
        .await
        .context("Failed to execute first update query")?;
    let query2 = sqlx::query!(
        r#"
        UPDATE quota_flavorquota
        SET flavor_group_id = ?
        WHERE quota_ptr_id = ?
        "#,
        flavor_group,
        data.id,
    );
    transaction
        .execute(query2)
        .await
        .context("Failed to execute second update query")?;
    let flavor_quota = FlavorQuota {
        id: data.id,
        user,
        username,
        quota,
        flavor_group,
        flavor_group_name,
    };
    Ok(flavor_quota)
}
