//! Implementation of the user-me endpoint.

use actix_web::{HttpResponse, web::ReqData};
use avina_wire::user::{Project, ProjectMinimal, User, UserDetailed};

/// Endpoint to return the calling user's information.
#[tracing::instrument(name = "user_me")]
pub async fn user_me(
    user: ReqData<User>,
    project: ReqData<Project>,
) -> HttpResponse {
    let user_detailed = UserDetailed {
        id: user.id,
        name: user.name.clone(),
        openstack_id: user.openstack_id.clone(),
        project: ProjectMinimal {
            id: project.id,
            name: project.name.clone(),
            user_class: project.user_class,
        },
        project_name: project.name.clone(),
        role: user.role,
        is_staff: user.is_staff,
        is_active: user.is_active,
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(user_detailed)
}
