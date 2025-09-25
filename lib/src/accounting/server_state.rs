use std::rc::Rc;

use anyhow::Context;
use avina_wire::accounting::{
    ServerState, ServerStateCreateData, ServerStateImport,
    ServerStateListParams, ServerStateModifyData,
};
use chrono::{DateTime, FixedOffset};
use reqwest::{Client, Method, StatusCode};
use uuid::Uuid;

use crate::{
    common::{SerializableNone, request, request_bare},
    error::ApiError,
};

#[derive(Debug)]
pub struct ServerStateApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct ServerStateListRequest {
    url: String,
    client: Rc<Client>,

    params: ServerStateListParams,
}

impl ServerStateListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: ServerStateListParams {
                server: None,
                user: None,
                project: None,
                all: None,
            },
        }
    }

    pub async fn send(&self) -> Result<Vec<ServerState>, ApiError> {
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

    pub fn server(&mut self, server: Uuid) -> &mut Self {
        self.params.server = Some(server);
        self
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.params.user = Some(user);
        self
    }

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.params.project = Some(project);
        self
    }

    pub fn all(&mut self) -> &mut Self {
        self.params.all = Some(true);
        self
    }
}

pub struct ServerStateCreateRequest {
    url: String,
    client: Rc<Client>,

    data: ServerStateCreateData,
}

impl ServerStateCreateRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        begin: DateTime<FixedOffset>,
        instance_id: Uuid,
        instance_name: String,
        flavor: u32,
        status: String,
        user: u32,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ServerStateCreateData::new(
                begin,
                instance_id,
                instance_name,
                flavor,
                status,
                user,
            ),
        }
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.data.end = Some(end);
        self
    }

    pub async fn send(&self) -> Result<ServerState, ApiError> {
        request(
            &self.client,
            Method::POST,
            &self.url,
            Some(&self.data),
            StatusCode::CREATED,
        )
        .await
    }
}

pub struct ServerStateModifyRequest {
    url: String,
    client: Rc<Client>,

    data: ServerStateModifyData,
}

impl ServerStateModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ServerStateModifyData::new(id),
        }
    }

    pub fn begin(&mut self, begin: DateTime<FixedOffset>) -> &mut Self {
        self.data.begin = Some(begin);
        self
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.data.end = Some(end);
        self
    }

    pub fn instance_id(&mut self, instance_id: Uuid) -> &mut Self {
        self.data.instance_id = Some(instance_id);
        self
    }

    pub fn instance_name(&mut self, instance_name: String) -> &mut Self {
        self.data.instance_name = Some(instance_name);
        self
    }

    pub fn flavor(&mut self, flavor: u32) -> &mut Self {
        self.data.flavor = Some(flavor);
        self
    }

    pub fn status(&mut self, status: String) -> &mut Self {
        self.data.status = Some(status);
        self
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.data.user = Some(user);
        self
    }

    pub async fn send(&self) -> Result<ServerState, ApiError> {
        request(
            &self.client,
            Method::PATCH,
            &self.url,
            Some(&self.data),
            StatusCode::OK,
        )
        .await
    }
}

impl ServerStateApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ServerStateApi {
        ServerStateApi {
            url: format!("{base_url}/accounting/serverstates"),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> ServerStateListRequest {
        ServerStateListRequest::new(self.url.as_ref(), &self.client)
    }

    pub async fn get(&self, id: u32) -> Result<ServerState, ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}", self.url, id);
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub fn create(
        &self,
        begin: DateTime<FixedOffset>,
        instance_id: Uuid,
        instance_name: String,
        flavor: u32,
        status: String,
        user: u32,
    ) -> ServerStateCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        ServerStateCreateRequest::new(
            url.as_ref(),
            &self.client,
            begin,
            instance_id,
            instance_name,
            flavor,
            status,
            user,
        )
    }

    pub fn modify(&self, id: u32) -> ServerStateModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        ServerStateModifyRequest::new(url.as_ref(), &self.client, id)
    }

    pub async fn delete(&self, id: u32) -> Result<(), ApiError> {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        request_bare(
            &self.client,
            Method::DELETE,
            url.as_str(),
            SerializableNone!(),
            StatusCode::NO_CONTENT,
        )
        .await?;
        Ok(())
    }

    pub async fn import(&self) -> Result<ServerStateImport, ApiError> {
        // TODO use Url.join
        let url = format!("{}/import/", self.url);
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
