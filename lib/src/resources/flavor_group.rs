use std::rc::Rc;

use anyhow::Context;
use avina_wire::resources::{
    FlavorGroup, FlavorGroupCreateData, FlavorGroupCreated,
    FlavorGroupDetailed, FlavorGroupInitialize, FlavorGroupListParams,
    FlavorGroupModifyData, FlavorGroupUsageAggregate, FlavorGroupUsageParams,
    FlavorGroupUsageSimple,
};
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request, request_bare},
    error::ApiError,
};

#[derive(Debug)]
pub struct FlavorGroupApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorGroupListRequest {
    url: String,
    client: Rc<Client>,

    params: FlavorGroupListParams,
}

impl FlavorGroupListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: FlavorGroupListParams { all: None },
        }
    }

    // TODO: only the return type changes, pull these functions into a macro
    pub async fn send(&self) -> Result<Vec<FlavorGroup>, ApiError> {
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

    pub fn all(&mut self) -> &mut Self {
        self.params.all = Some(true);
        self
    }
}

pub struct FlavorGroupCreateRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorGroupCreateData,
}

impl FlavorGroupCreateRequest {
    pub fn new(url: &str, client: &Rc<Client>, name: String) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorGroupCreateData::new(name),
        }
    }

    pub async fn send(&self) -> Result<FlavorGroupCreated, ApiError> {
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

pub struct FlavorGroupModifyRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorGroupModifyData,
}

impl FlavorGroupModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorGroupModifyData::new(id),
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.data.name = Some(name);
        self
    }

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.data.project = Some(project);
        self
    }

    pub async fn send(&self) -> Result<FlavorGroup, ApiError> {
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

pub struct FlavorGroupUsageRequest {
    url: String,
    client: Rc<Client>,

    params: FlavorGroupUsageParams,
}

impl FlavorGroupUsageRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            // TODO: shouldn't we be able to use ::default() in these cases
            params: FlavorGroupUsageParams {
                user: None,
                project: None,
                all: None,
                aggregate: None,
            },
        }
    }

    pub async fn user(
        &mut self,
        user: u32,
    ) -> Result<Vec<FlavorGroupUsageSimple>, ApiError> {
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

    pub async fn user_aggregate(
        &mut self,
        user: u32,
    ) -> Result<Vec<FlavorGroupUsageAggregate>, ApiError> {
        self.params.user = Some(user);
        self.params.aggregate = Some(true);
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
    ) -> Result<Vec<FlavorGroupUsageSimple>, ApiError> {
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

    pub async fn project_aggregate(
        &mut self,
        project: u32,
    ) -> Result<Vec<FlavorGroupUsageAggregate>, ApiError> {
        self.params.project = Some(project);
        self.params.aggregate = Some(true);
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

    pub async fn all(
        &mut self,
    ) -> Result<Vec<FlavorGroupUsageSimple>, ApiError> {
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

    pub async fn all_aggregate(
        &mut self,
    ) -> Result<Vec<FlavorGroupUsageAggregate>, ApiError> {
        self.params.all = Some(true);
        self.params.aggregate = Some(true);
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

    pub async fn mine(
        &mut self,
    ) -> Result<Vec<FlavorGroupUsageSimple>, ApiError> {
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

    pub async fn mine_aggregate(
        &mut self,
    ) -> Result<Vec<FlavorGroupUsageAggregate>, ApiError> {
        self.params.aggregate = Some(true);
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

impl FlavorGroupApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorGroupApi {
        FlavorGroupApi {
            url: format!("{base_url}/resources/flavorgroups"),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorGroupListRequest {
        FlavorGroupListRequest::new(self.url.as_ref(), &self.client)
    }

    pub async fn get(&self, id: u32) -> Result<FlavorGroupDetailed, ApiError> {
        // TODO: use Url.join
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

    pub fn create(&self, name: String) -> FlavorGroupCreateRequest {
        // TODO: use Url.join
        let url = format!("{}/", self.url);
        FlavorGroupCreateRequest::new(url.as_ref(), &self.client, name)
    }

    pub fn modify(&self, id: u32) -> FlavorGroupModifyRequest {
        // TODO: use Url.join
        let url = format!("{}/{}/", self.url, id);
        FlavorGroupModifyRequest::new(url.as_ref(), &self.client, id)
    }

    pub async fn delete(&self, id: u32) -> Result<(), ApiError> {
        // TODO: use Url.join
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

    pub async fn initialize(&self) -> Result<FlavorGroupInitialize, ApiError> {
        // TODO: use Url.join
        let url = format!("{}/initialize/", self.url);
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub fn usage(&self) -> FlavorGroupUsageRequest {
        let url = format!("{}/usage/", self.url);
        FlavorGroupUsageRequest::new(url.as_ref(), &self.client)
    }
}
