use std::{fmt::Debug, rc::Rc};

use anyhow::Context;
use avina_wire::accounting::{
    ServerCostAll, ServerCostParams, ServerCostProject, ServerCostServer,
    ServerCostSimple, ServerCostUser,
};
use chrono::{DateTime, FixedOffset};
use reqwest::{Client, Method, StatusCode};
use uuid::Uuid;

use crate::{
    common::{SerializableNone, request},
    error::ApiError,
};

#[derive(Debug)]
pub struct ServerCostRequest {
    url: String,
    client: Rc<Client>,

    params: ServerCostParams,
}

impl ServerCostRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            // TODO: we should be able to use Default the *Params inits
            params: ServerCostParams {
                begin: None,
                end: None,
                server: None,
                user: None,
                project: None,
                all: None,
                detail: None,
            },
        }
    }

    pub fn begin(&mut self, begin: DateTime<FixedOffset>) -> &mut Self {
        self.params.begin = Some(begin);
        self
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.params.end = Some(end);
        self
    }

    pub async fn server(
        &mut self,
        server: Uuid,
    ) -> Result<ServerCostSimple, ApiError> {
        self.params.server = Some(server);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn server_detail(
        &mut self,
        server: Uuid,
    ) -> Result<ServerCostServer, ApiError> {
        self.params.server = Some(server);
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn user(
        &mut self,
        user: u32,
    ) -> Result<ServerCostSimple, ApiError> {
        self.params.user = Some(user);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn user_detail(
        &mut self,
        user: u32,
    ) -> Result<ServerCostUser, ApiError> {
        self.params.user = Some(user);
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn project(
        &mut self,
        project: u32,
    ) -> Result<ServerCostSimple, ApiError> {
        self.params.project = Some(project);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn project_detail(
        &mut self,
        project: u32,
    ) -> Result<ServerCostProject, ApiError> {
        self.params.project = Some(project);
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn all(&mut self) -> Result<ServerCostSimple, ApiError> {
        self.params.all = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn all_detail(&mut self) -> Result<ServerCostAll, ApiError> {
        self.params.all = Some(true);
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn mine(&mut self) -> Result<ServerCostSimple, ApiError> {
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn mine_detail(&mut self) -> Result<ServerCostUser, ApiError> {
        self.params.detail = Some(true);
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters")?;
        let url = if params.is_empty() {
            self.url.clone()
        } else {
            format!("{}?{}", self.url, params)
        };
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }
}

#[derive(Debug)]
pub struct ServerCostApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl ServerCostApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ServerCostApi {
        ServerCostApi {
            url: format!("{base_url}/accounting/servercost/"),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> ServerCostRequest {
        ServerCostRequest::new(self.url.as_str(), &self.client)
    }
}
