use std::rc::Rc;

use anyhow::Context;
use avina_wire::resources::{
    Flavor, FlavorCreateData, FlavorDetailed, FlavorImport, FlavorListParams,
    FlavorModifyData, FlavorUsageAggregate, FlavorUsageParams,
    FlavorUsageSimple,
};
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request, request_bare},
    error::ApiError,
};

#[derive(Debug)]
pub struct FlavorApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorListRequest {
    url: String,
    client: Rc<Client>,

    params: FlavorListParams,
}

impl FlavorListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: FlavorListParams {
                all: None,
                group: None,
            },
        }
    }

    pub fn all(&mut self) -> &mut Self {
        self.params.all = Some(true);
        self
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.params.group = Some(group);
        self
    }

    pub async fn send(&self) -> Result<Vec<Flavor>, ApiError> {
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

pub struct FlavorCreateRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorCreateData,
}

impl FlavorCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        name: String,
        openstack_id: String,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorCreateData::new(name, openstack_id),
        }
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.data.group = Some(group);
        self
    }

    pub fn weight(&mut self, weight: u32) -> &mut Self {
        self.data.weight = Some(weight);
        self
    }

    pub async fn send(&self) -> Result<FlavorDetailed, ApiError> {
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

pub struct FlavorModifyRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorModifyData,
}

impl FlavorModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorModifyData::new(id),
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.data.name = Some(name);
        self
    }

    pub fn openstack_id(&mut self, openstack_id: String) -> &mut Self {
        self.data.openstack_id = Some(openstack_id);
        self
    }

    pub fn group(&mut self, group: u32) -> &mut Self {
        self.data.group = Some(Some(group));
        self
    }

    pub fn no_group(&mut self) -> &mut Self {
        self.data.group = Some(None);
        self
    }

    pub fn weight(&mut self, weight: u32) -> &mut Self {
        self.data.weight = Some(weight);
        self
    }

    pub async fn send(&self) -> Result<Flavor, ApiError> {
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

pub struct FlavorUsageRequest {
    url: String,
    client: Rc<Client>,

    params: FlavorUsageParams,
}

impl FlavorUsageRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: FlavorUsageParams {
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
    ) -> Result<Vec<FlavorUsageSimple>, ApiError> {
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
    ) -> Result<Vec<FlavorUsageAggregate>, ApiError> {
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
    ) -> Result<Vec<FlavorUsageSimple>, ApiError> {
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
    ) -> Result<Vec<FlavorUsageAggregate>, ApiError> {
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

    pub async fn all(&mut self) -> Result<Vec<FlavorUsageSimple>, ApiError> {
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
    ) -> Result<Vec<FlavorUsageAggregate>, ApiError> {
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

    pub async fn mine(&mut self) -> Result<Vec<FlavorUsageSimple>, ApiError> {
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

    // TODO this causes a http 500 error
    pub async fn mine_aggregate(
        &mut self,
    ) -> Result<Vec<FlavorUsageAggregate>, ApiError> {
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

impl FlavorApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorApi {
        FlavorApi {
            url: format!("{base_url}/resources/flavors"),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorListRequest {
        FlavorListRequest::new(self.url.as_ref(), &self.client)
    }

    pub async fn get(&self, id: u32) -> Result<FlavorDetailed, ApiError> {
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
        name: String,
        openstack_id: String,
    ) -> FlavorCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        FlavorCreateRequest::new(url.as_ref(), &self.client, name, openstack_id)
    }

    pub fn modify(&self, id: u32) -> FlavorModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        FlavorModifyRequest::new(url.as_ref(), &self.client, id)
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

    pub async fn import(&self) -> Result<FlavorImport, ApiError> {
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

    pub fn usage(&self) -> FlavorUsageRequest {
        let url = format!("{}/usage/", self.url);
        FlavorUsageRequest::new(url.as_ref(), &self.client)
    }
}
