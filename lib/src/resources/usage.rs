use std::rc::Rc;

use avina_wire::resources::CloudUsage;
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request},
    error::ApiError,
};

#[derive(Debug)]
pub struct UsageApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl UsageApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> UsageApi {
        UsageApi {
            // TODO: add the missing / that the end
            url: format!("{base_url}/resources/usage"),
            client: Rc::clone(client),
        }
    }

    pub async fn get(&self) -> Result<CloudUsage, ApiError> {
        request(
            &self.client,
            Method::GET,
            self.url.as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }
}
