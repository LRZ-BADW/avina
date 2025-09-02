use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
};
use avina_wire::user::{Project, User};
use sqlx::MySqlPool;

use crate::{
    error::{bad_request_error, internal_server_error, unauthorized_error},
    openstack::{OpenStack, ProjectMinimal as OpenstackProjectMinimal},
};

// TODO revise error handling here as well and test errors

#[derive(Clone, Debug)]
pub struct Token(pub String);

pub async fn require_valid_token(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let Some(token) = req.headers().get("X-Auth-Token") else {
        return Err(unauthorized_error("No token in request header"));
    };
    let Ok(token) = token.to_str() else {
        return Err(bad_request_error("Token is not a valid string"));
    };
    let Some(openstack) = req.app_data::<Data<OpenStack>>() else {
        return Err(internal_server_error(
            "No OpenStack client in application state",
        ));
    };
    let Ok(os_project) = openstack.validate_user_token(token).await else {
        return Err(unauthorized_error("Failed to validate user token"));
    };
    req.extensions_mut().insert(Token(token.into()));
    req.extensions_mut().insert(os_project);
    next.call(req).await
}

pub async fn extract_user_and_project(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let os_project = match req.extensions().get::<OpenstackProjectMinimal>() {
        Some(os_project) => os_project.clone(),
        None => {
            return Err(internal_server_error(
                "No OpenStack project in request extensions",
            ));
        }
    };
    let Some(db_pool) = req.app_data::<Data<MySqlPool>>() else {
        return Err(internal_server_error(
            "No database connection pool in application state",
        ));
    };

    struct Row {
        user_id: i32,
        user_name: String,
        user_openstack_id: String,
        user_role: u32,
        user_is_staff: i8,
        user_is_active: i8,
        project_id: i32,
        project_name: String,
        project_openstack_id: String,
        project_user_class: u32,
    }

    let Ok(row) = sqlx::query_as!(
        Row,
        r#"
        SELECT
            user.id AS user_id,
            user.name AS user_name,
            user.openstack_id AS user_openstack_id,
            user.role AS user_role,
            user.is_staff AS user_is_staff,
            user.is_active AS user_is_active,
            project.id AS project_id,
            project.name AS project_name,
            project.openstack_id AS project_openstack_id,
            project.user_class AS project_user_class
        FROM user_user AS user, user_project AS project
        WHERE
            user.project_id = project.id AND
            user.name = ?
        "#,
        os_project.name
    )
    .fetch_one(db_pool.get_ref())
    .await
    else {
        // TODO apply context and map_err
        return Err(unauthorized_error(
            "Failed to retrieve user and project from database",
        ));
    };

    let user = User {
        id: row.user_id as u32,
        name: row.user_name,
        openstack_id: row.user_openstack_id,
        project: row.project_id as u32,
        project_name: row.project_name.clone(),
        role: row.user_role,
        is_staff: row.user_is_staff != 0,
        is_active: row.user_is_active != 0,
    };

    let user_class = match row.project_user_class.try_into() {
        Ok(u) => u,
        Err(_) => {
            return Err(internal_server_error("Failed to parse user class"));
        }
    };

    let project = Project {
        id: row.project_id as u32,
        name: row.project_name,
        openstack_id: row.project_openstack_id,
        user_class,
    };

    req.extensions_mut().insert(user);
    req.extensions_mut().insert(project);

    next.call(req).await
}
