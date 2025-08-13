use std::rc::Rc;

use anyhow::Context;
use avina_wire::budgeting::{BudgetOverTree, BudgetOverTreeParams};
use chrono::{DateTime, FixedOffset};
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request},
    error::ApiError,
};

#[derive(Debug)]
pub struct BudgetOverTreeApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct BudgetOverTreeRequest {
    url: String,
    client: Rc<Client>,

    params: BudgetOverTreeParams,
}

impl BudgetOverTreeRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: BudgetOverTreeParams {
                all: None,
                project: None,
                user: None,
                end: None,
            },
        }
    }

    pub async fn send(&self) -> Result<BudgetOverTree, ApiError> {
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

    pub fn project(&mut self, project: u32) -> &mut Self {
        self.params.project = Some(project);
        self
    }

    pub fn user(&mut self, user: u32) -> &mut Self {
        self.params.user = Some(user);
        self
    }

    pub fn end(&mut self, end: DateTime<FixedOffset>) -> &mut Self {
        self.params.end = Some(end);
        self
    }
}

impl BudgetOverTreeApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> BudgetOverTreeApi {
        BudgetOverTreeApi {
            url: format!("{base_url}/budgeting/budgetovertree/"),
            client: Rc::clone(client),
        }
    }

    pub fn get(&self) -> BudgetOverTreeRequest {
        BudgetOverTreeRequest::new(&self.url, &self.client)
    }
}
