use std::rc::Rc;

use anyhow::Context;
use avina_wire::user::{
    Project, ProjectCreateData, ProjectListParams, ProjectModifyData,
    ProjectRetrieved, UserClass,
};
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request, request_bare},
    error::ApiError,
};

#[derive(Debug)]
pub struct ProjectApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct ProjectListRequest {
    url: String,
    client: Rc<Client>,

    params: ProjectListParams,
}

impl ProjectListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            params: ProjectListParams {
                all: None,
                userclass: None,
            },
        }
    }

    pub async fn send(&self) -> Result<Vec<Project>, ApiError> {
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters.")?;
        // TODO: maybe use url join
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

    pub fn user_class(&mut self, userclass: UserClass) -> &mut Self {
        self.params.userclass = Some(userclass);
        self
    }
}

pub struct ProjectCreateRequest {
    url: String,
    client: Rc<Client>,

    data: ProjectCreateData,
}

impl ProjectCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        name: String,
        openstack_id: String,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ProjectCreateData::new(name, openstack_id),
        }
    }

    pub fn user_class(&mut self, user_class: UserClass) -> &mut Self {
        self.data.user_class = Some(user_class);
        self
    }

    pub async fn send(&self) -> Result<Project, ApiError> {
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

pub struct ProjectModifyRequest {
    url: String,
    client: Rc<Client>,

    data: ProjectModifyData,
}

impl ProjectModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ProjectModifyData::new(id),
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

    pub fn user_class(&mut self, user_class: UserClass) -> &mut Self {
        self.data.user_class = Some(user_class);
        self
    }

    pub async fn send(&self) -> Result<Project, ApiError> {
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

impl ProjectApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ProjectApi {
        ProjectApi {
            url: format!("{base_url}/user/projects"),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> ProjectListRequest {
        ProjectListRequest::new(self.url.as_ref(), &self.client)
    }

    pub async fn get(&self, id: u32) -> Result<ProjectRetrieved, ApiError> {
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
    ) -> ProjectCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        ProjectCreateRequest::new(
            url.as_ref(),
            &self.client,
            name,
            openstack_id,
        )
    }

    pub fn modify(&self, id: u32) -> ProjectModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        ProjectModifyRequest::new(url.as_ref(), &self.client, id)
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
}
