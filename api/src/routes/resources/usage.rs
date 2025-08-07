use std::collections::HashMap;

use actix_web::{
    HttpResponse, Scope, http,
    web::{Data, ReqData, get, scope},
};
use avina_wire::user::User;
use reqwest::Client;

use crate::{
    authentication::Token, error::AuthOnlyError, startup::CloudUsageUrl,
};

pub fn usage_scope() -> Scope {
    scope("/usage").route("", get().to(cloud_usage))
}

#[tracing::instrument(name = "cloud_usage")]
async fn cloud_usage(
    user: ReqData<User>,
    token: ReqData<Token>,
    cloud_usage_url: Data<CloudUsageUrl>,
) -> Result<HttpResponse, AuthOnlyError> {
    let Some(url) = cloud_usage_url.as_ref().0.clone() else {
        tracing::error!("Cloud usage URL is not configured.");
        return Ok(HttpResponse::InternalServerError().finish());
    };
    let mut data = HashMap::new();
    data.insert("project_id", user.openstack_id.clone());
    data.insert("token_id", token.0.clone());
    let Ok(response) = Client::new().get(url).json(&data).send().await else {
        tracing::error!("Call to Cloud Usage backend failed.");
        return Ok(HttpResponse::InternalServerError().finish());
    };
    if response.status().as_u16() != http::StatusCode::OK {
        tracing::error!("Cloud Usage backend returned unexpected status code.");
        return Ok(HttpResponse::InternalServerError().finish());
    }
    let Ok(body) = response.text().await else {
        tracing::error!("Unable to get body from Cloud Usage response.");
        return Ok(HttpResponse::InternalServerError().finish());
    };
    Ok(HttpResponse::Ok().body(body))
}
