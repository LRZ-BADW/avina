use std::rc::Rc;

use anyhow::Context;
use avina_wire::{
    pricing::{
        FlavorPrice, FlavorPriceCreateData, FlavorPriceInitialize,
        FlavorPriceListParams, FlavorPriceModifyData,
    },
    user::UserClass,
};
use chrono::{DateTime, FixedOffset};
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request, request_bare},
    error::ApiError,
};

#[derive(Debug)]
pub struct FlavorPriceApi {
    pub url: String,
    pub client: Rc<Client>,
}

#[derive(Debug)]
pub struct FlavorPriceListRequest {
    url: String,
    client: Rc<Client>,

    params: FlavorPriceListParams,
}

impl FlavorPriceListRequest {
    pub fn new(url: &str, client: &Rc<Client>) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),

            params: FlavorPriceListParams {
                user_class: None,
                current: None,
            },
        }
    }

    pub fn user_class(&mut self, user_class: UserClass) -> &mut Self {
        self.params.user_class = Some(user_class);
        self
    }

    pub fn current(&mut self) -> &mut Self {
        self.params.current = Some(true);
        self
    }

    pub async fn send(&self) -> Result<Vec<FlavorPrice>, ApiError> {
        let params = serde_urlencoded::to_string(&self.params)
            .context("Failed to encode URL parameters.")?;
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

pub struct FlavorPriceCreateRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorPriceCreateData,
}

impl FlavorPriceCreateRequest {
    pub fn new(
        url: &str,
        client: &Rc<Client>,
        flavor: u32,
        user_class: UserClass,
    ) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorPriceCreateData::new(flavor, user_class),
        }
    }

    pub fn price(&mut self, price: f64) -> &mut Self {
        self.data.price = Some(price);
        self
    }

    pub fn start_time(
        &mut self,
        start_time: DateTime<FixedOffset>,
    ) -> &mut Self {
        self.data.start_time = Some(start_time);
        self
    }

    pub async fn send(&self) -> Result<FlavorPrice, ApiError> {
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

pub struct FlavorPriceModifyRequest {
    url: String,
    client: Rc<Client>,

    data: FlavorPriceModifyData,
}

impl FlavorPriceModifyRequest {
    pub fn new(url: &str, client: &Rc<Client>, id: u32) -> Self {
        Self {
            url: url.to_string(),
            client: Rc::clone(client),
            data: FlavorPriceModifyData::new(id),
        }
    }

    pub fn flavor(&mut self, flavor: u32) -> &mut Self {
        self.data.flavor = Some(flavor);
        self
    }

    pub fn user_class(&mut self, user_class: UserClass) -> &mut Self {
        self.data.user_class = Some(user_class);
        self
    }

    pub fn unit_price(&mut self, unit_price: f64) -> &mut Self {
        self.data.unit_price = Some(unit_price);
        self
    }

    pub fn start_time(
        &mut self,
        start_time: DateTime<FixedOffset>,
    ) -> &mut Self {
        self.data.start_time = Some(start_time);
        self
    }

    pub async fn send(&self) -> Result<FlavorPrice, ApiError> {
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

impl FlavorPriceApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> FlavorPriceApi {
        FlavorPriceApi {
            url: format!("{base_url}/pricing/flavorprices"),
            client: Rc::clone(client),
        }
    }

    pub fn list(&self) -> FlavorPriceListRequest {
        FlavorPriceListRequest::new(self.url.as_ref(), &self.client)
    }

    pub async fn get(&self, id: u32) -> Result<FlavorPrice, ApiError> {
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

    pub fn create(
        &self,
        flavor: u32,
        user_class: UserClass,
    ) -> FlavorPriceCreateRequest {
        // TODO: use Url.join
        let url = format!("{}/", self.url);
        FlavorPriceCreateRequest::new(
            url.as_ref(),
            &self.client,
            flavor,
            user_class,
        )
    }

    pub fn modify(&self, id: u32) -> FlavorPriceModifyRequest {
        // TODO: use Url.join
        let url = format!("{}/{}/", self.url, id);
        FlavorPriceModifyRequest::new(url.as_ref(), &self.client, id)
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

    pub async fn initialize(&self) -> Result<FlavorPriceInitialize, ApiError> {
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
}
