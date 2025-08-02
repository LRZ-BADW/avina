use std::rc::Rc;

use avina_wire::bill::Bill;
use reqwest::{Client, Method, StatusCode};

use crate::{
    common::{SerializableNone, request},
    error::ApiError,
};

#[derive(Debug)]
pub struct BillApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl BillApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> BillApi {
        BillApi {
            url: format!("{base_url}/bill"),
            client: Rc::clone(client),
        }
    }

    pub async fn get(&self) -> Result<Bill, ApiError> {
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
