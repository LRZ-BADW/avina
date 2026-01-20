use std::rc::Rc;

use anyhow::Context;
use avina_wire::user::{
    User, UserCreateData, UserDetailed, UserImport, UserListParams,
    UserModifyData, UserSync,
};
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request, request_bare},
    error::ApiError,
};

#[derive(Debug)]
pub struct UserApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct UserListRequest {
    url: String,
    client: Rc<Client>,

    params: UserListParams,
}

impl UserListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: UserListParams {
                all: None,
                project: None,
            },
        }
    }

    pub async fn send(&self) -> Result<Vec<User>, ApiError> {
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

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.params.project = Some(project);
        self
    }
}

pub struct UserCreateRequest {
    url: String,
    client: Rc<Client>,

    data: UserCreateData,
}

impl UserCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        name: String,
        openstack_id: String,
        project: u32,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: UserCreateData::new(name, openstack_id, project),
        }
    }

    pub fn role(&mut self, role: u32) -> &mut Self {
        self.data.role = Some(role);
        self
    }

    pub fn staff(&mut self) -> &mut Self {
        self.data.is_staff = Some(true);
        self
    }

    pub fn inactive(&mut self) -> &mut Self {
        self.data.is_active = Some(false);
        self
    }

    pub async fn send(&self) -> Result<User, ApiError> {
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

pub struct UserModifyRequest {
    url: String,
    client: Rc<Client>,

    data: UserModifyData,
}

impl UserModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: UserModifyData::new(id),
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

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.data.project = Some(project);
        self
    }

    pub fn role(&mut self, role: u32) -> &mut Self {
        self.data.role = Some(role);
        self
    }

    pub fn is_staff(&mut self, is_staff: bool) -> &mut Self {
        self.data.is_staff = Some(is_staff);
        self
    }

    pub fn is_active(&mut self, is_active: bool) -> &mut Self {
        self.data.is_active = Some(is_active);
        self
    }

    pub async fn send(&self) -> Result<User, ApiError> {
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

impl UserApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> UserApi {
        UserApi {
            url: format!("{base_url}/user/users"),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> UserListRequest {
        UserListRequest::new(self.url.as_ref(), &self.client)
    }

    pub async fn get(&self, id: u32) -> Result<UserDetailed, ApiError> {
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
        project: u32,
    ) -> UserCreateRequest {
        // TODO use Url.join
        let url = format!("{}/", self.url);
        UserCreateRequest::new(
            url.as_ref(),
            &self.client,
            name,
            openstack_id,
            project,
        )
    }

    pub fn modify(&self, id: u32) -> UserModifyRequest {
        // TODO use Url.join
        let url = format!("{}/{}/", self.url, id);
        UserModifyRequest::new(url.as_ref(), &self.client, id)
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

    pub async fn me(&self) -> Result<UserDetailed, ApiError> {
        // TODO use Url.join
        let url = format!(
            "{}/me",
            self.url
                .rfind('/')
                .map(|i| &self.url[..i])
                .unwrap_or(&self.url)
        );
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn import(&self) -> Result<UserImport, ApiError> {
        // TODO use Url.join
        let url = format!(
            "{}/import",
            self.url
                .rfind('/')
                .map(|i| &self.url[..i])
                .unwrap_or(&self.url)
        );
        request(
            &self.client,
            Method::GET,
            url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    pub async fn sync(&self) -> Result<UserSync, ApiError> {
        // TODO use Url.join
        let url = format!(
            "{}/sync",
            self.url
                .rfind('/')
                .map(|i| &self.url[..i])
                .unwrap_or(&self.url)
        );
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
