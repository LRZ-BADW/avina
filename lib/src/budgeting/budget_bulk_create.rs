use std::rc::Rc;

use avina_wire::budgeting::{BudgetBulkCreate, BudgetBulkCreateData};
use reqwest::{Client, Method, StatusCode};

use crate::{common::request, error::ApiError};

#[derive(Debug)]
pub struct BudgetBulkCreateApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl BudgetBulkCreateApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> BudgetBulkCreateApi {
        BudgetBulkCreateApi {
            url: format!("{base_url}/budgeting/budgetbulkcreate/"),
            client: Rc::clone(client),
        }
    }

    pub async fn run(&self, year: i32) -> Result<BudgetBulkCreate, ApiError> {
        request(
            &self.client,
            Method::POST,
            self.url.as_str(),
            Some(&BudgetBulkCreateData { year }),
            StatusCode::OK,
        )
        .await
    }
}
