//! Bindings for hello endpoints of the API.

use std::rc::Rc;

use avina_wire::hello::Hello;
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request},
    error::ApiError,
};

/// Client for the hello module of the API.
#[derive(Debug)]
pub struct HelloApi {
    /// URL of the hello module of the API.
    pub url: String,
    /// Reference to the HTTP client.
    pub client: Rc<Client>,
}

impl HelloApi {
    /// Build a new instance from the given base URL and HTTP client reference.
    pub fn new(base_url: &str, client: &Rc<Client>) -> HelloApi {
        HelloApi {
            url: format!("{base_url}/hello"),
            client: Rc::clone(client),
        }
    }

    /// Get a hello message for admin users.
    pub async fn admin(&self) -> Result<Hello, ApiError> {
        request(
            &self.client,
            Method::GET,
            format!("{}/admin", self.url).as_str(),
            SerializableNone!(),
            StatusCode::OK,
        )
        .await
    }

    /// Get a hello message for normal users.
    pub async fn user(&self) -> Result<Hello, ApiError> {
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
