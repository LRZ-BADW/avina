use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use anyhow::Context;
use avina_wire::user::{User, UserSync};
use sqlx::MySqlPool;

use crate::{
    authorization::require_admin_user,
    database::user::user::{select_all_users_from_db, update_user_role_in_db},
    error::NormalApiError,
    ldap::AvinaLdap,
    startup::AvinaLdapConfig,
};

#[tracing::instrument(name = "user_sync")]
pub async fn user_sync(
    user: ReqData<User>,
    db_pool: Data<MySqlPool>,
    avina_ldap_config: Data<AvinaLdapConfig>,
) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;
    let mut transaction = db_pool
        .begin()
        .await
        .context("Failed to begin transaction")?;

    let ldap_data = AvinaLdap::new(&avina_ldap_config).await?;

    let users = select_all_users_from_db(&mut transaction).await?;
    let mut updated_user_count = 0;
    for user in users {
        let role = ldap_data.get_role(&user.name);
        if role != user.role {
            update_user_role_in_db(&mut transaction, user.id, role)
                .await
                .context(format!(
                    "Could not update role of user {}.",
                    user.name
                ))?;
            updated_user_count += 1;
        }
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(UserSync {
            updated_project_count: 0,
            updated_user_count,
        }))
}
