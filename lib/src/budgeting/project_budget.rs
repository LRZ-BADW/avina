use std::rc::Rc;

use anyhow::Context;
use avina_wire::budgeting::{
    ProjectBudget, ProjectBudgetCreateData, ProjectBudgetListParams,
    ProjectBudgetModifyData, ProjectBudgetOverDetail, ProjectBudgetOverParams,
    ProjectBudgetOverSimple,
};
use chrono::{DateTime, FixedOffset};
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request, request_bare},
    error::ApiError,
};

#[derive(Debug)]
pub struct ProjectBudgetApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct ProjectBudgetListRequest {
    url: String,
    client: Rc<Client>,

    params: ProjectBudgetListParams,
}

impl ProjectBudgetListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: ProjectBudgetListParams {
                user: None,
                project: None,
                all: None,
                year: None,
            },
        }
    }

    pub async fn send(&self) -> Result<Vec<ProjectBudget>, ApiError> {
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

    pub fn year(&mut self, year: u32) -> &mut Self {
        self.params.year = Some(year);
        self
    }
}

pub struct ProjectBudgetCreateRequest {
    url: String,
    client: Rc<Client>,

    data: ProjectBudgetCreateData,
}

impl ProjectBudgetCreateRequest {
    pub fn new(url: &str, client: &Rc<Client>, project: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ProjectBudgetCreateData::new(project),
        }
    }

    pub fn year(&mut self, year: u32) -> &mut Self {
        self.data.year = Some(year);
        self
    }

    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.data.amount = Some(amount);
        self
    }

    pub async fn send(&self) -> Result<ProjectBudget, ApiError> {
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

pub struct ProjectBudgetModifyRequest {
    url: String,
    client: Rc<Client>,

    data: ProjectBudgetModifyData,
}

impl ProjectBudgetModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: ProjectBudgetModifyData::new(id),
        }
    }

    pub fn amount(&mut self, amount: u32) -> &mut Self {
        self.data.amount = Some(amount);
        self
    }

    pub fn force(&mut self) -> &mut Self {
        self.data.force = true;
        self
    }

    pub async fn send(&self) -> Result<ProjectBudget, ApiError> {
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

#[derive(Debug)]
pub struct ProjectBudgetOverRequest {
    url: String,
    client: Rc<Client>,

    params: ProjectBudgetOverParams,
}

impl ProjectBudgetOverRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: ProjectBudgetOverParams {
                end: None,
                budget: None,
                project: None,
                all: None,
                detail: None,
            },
        }
    }

    pub async fn send(&self) -> Result<Vec<ProjectBudgetOverSimple>, ApiError> {
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

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.params.end = Some(end);
        self
    }

    pub fn budget(&mut self, budget: u32) -> &mut Self {
        self.params.budget = Some(budget);
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

    pub async fn normal(
        &mut self,
    ) -> Result<Vec<ProjectBudgetOverSimple>, ApiError> {
        self.params.detail = Some(false);
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

    pub async fn detail(
        &mut self,
    ) -> Result<Vec<ProjectBudgetOverDetail>, ApiError> {
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

impl ProjectBudgetApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> ProjectBudgetApi {
        ProjectBudgetApi {
            url: format!("{base_url}/budgeting/projectbudgets"),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> ProjectBudgetListRequest {
        ProjectBudgetListRequest::new(self.url.as_ref(), &self.client)
    }

    pub async fn get(&self, id: u32) -> Result<ProjectBudget, ApiError> {
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

    pub fn create(&self, project: u32) -> ProjectBudgetCreateRequest {
        // TODO: use Url.join
        let url = format!("{}/", self.url);
        ProjectBudgetCreateRequest::new(url.as_ref(), &self.client, project)
    }

    pub fn modify(&self, id: u32) -> ProjectBudgetModifyRequest {
        // TODO: use Url.join
        let url = format!("{}/{}/", self.url, id);
        ProjectBudgetModifyRequest::new(url.as_ref(), &self.client, id)
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

    pub fn over(&self) -> ProjectBudgetOverRequest {
        let url = format!("{}/over/", self.url);
        ProjectBudgetOverRequest::new(url.as_ref(), &self.client)
    }
}
