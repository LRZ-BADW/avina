//! Bindings for the [avina-api].
//!
//! The main types of this crate are [Token] which represents a OpenStack Keystone token, and [Api]
//! which can be created with it, and represents a client to the [avina-api]. [Api] contains
//! further `...Api` structs for all the respective modules of the API, e.g. [UserApi] in the
//! [user](Api::user) field.
//!
//! Those structs have member functions to call individual endpoints, e.g. [UserApi::get]. Endpoint
//! bindings that take optional arguments use the [builder pattern]. This allows mnemonic calls
//! like `api.user.list().all().send().await`. The calls return datatypes from the [avina_wire]
//! crate.
//!
//! # Usage
//!
//! Include the following in for `Cargo.toml`:
//!
//! ```toml
//! avina = 2
//! ```
//!
//! Then you can use the bindings like this:
//!
//! ```rust
//! use avina::{Token, Api};
//!
//! // let token = Token::from_str("abcdefg...").unwrap();
//! let token = Token::new(
//!                 auth_url.as_str(),
//!                 username.as_str(),
//!                 password.as_str(),
//!                 project_name.as_str(),
//!                 user_domain_name.as_str(),
//!                 project_domain_id.as_str(),
//!             ).unwrap();
//! let api = Api::new("https://cc.lrz.de:1338/api", token, None, None).unwrap();
//! println!("{:?}", api.user.me());
//! ```
//!
//! [avina-api]: https://docs.rs/avina-api
//! [builder pattern]: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html

use std::{rc::Rc, time::Duration};

use anyhow::Context;
use reqwest::{
    ClientBuilder,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};

pub mod common;
pub mod error;
use error::ApiError;

pub mod token;
pub use token::Token;

#[cfg(feature = "accounting")]
pub mod accounting;
#[cfg(feature = "budgeting")]
pub mod budgeting;
#[cfg(feature = "hello")]
pub mod hello;
#[cfg(feature = "pricing")]
pub mod pricing;
#[cfg(feature = "quota")]
pub mod quota;
#[cfg(feature = "resources")]
pub mod resources;
#[cfg(feature = "user")]
pub mod user;

#[cfg(feature = "accounting")]
use accounting::ServerConsumptionApi;
#[cfg(feature = "accounting")]
use accounting::ServerCostApi;
#[cfg(feature = "accounting")]
use accounting::ServerStateApi;
#[cfg(feature = "budgeting")]
use budgeting::BudgetBulkCreateApi;
#[cfg(feature = "budgeting")]
use budgeting::BudgetOverTreeApi;
#[cfg(feature = "budgeting")]
use budgeting::ProjectBudgetApi;
#[cfg(feature = "budgeting")]
use budgeting::UserBudgetApi;
#[cfg(feature = "hello")]
use hello::HelloApi;
#[cfg(feature = "pricing")]
use pricing::FlavorPriceApi;
#[cfg(feature = "quota")]
use quota::FlavorQuotaApi;
#[cfg(feature = "resources")]
use resources::FlavorApi;
#[cfg(feature = "resources")]
use resources::FlavorGroupApi;
#[cfg(feature = "resources")]
use resources::UsageApi;
#[cfg(feature = "user")]
use user::ProjectApi;
#[cfg(feature = "user")]
use user::UserApi;

/// Default timeout of HTTP calls in seconds (5 minutes)
pub const DEFAULT_TIMEOUT: u64 = 300;

/// Main API client.
///
/// This is a collection of all the individual API module clients.
/// While the token is held in this main struct, both the URL
/// and the references to a shared HTTP client are held in the
/// individual module clients.
#[derive(Debug)]
pub struct Api {
    /// Authentication token.
    #[allow(unused)]
    token: Token,
    /// Client for hello endpoints.
    #[cfg(feature = "hello")]
    pub hello: HelloApi,
    /// Client for project endpoints.
    #[cfg(feature = "user")]
    pub project: ProjectApi,
    /// Client for user endpoints.
    #[cfg(feature = "user")]
    pub user: UserApi,
    /// Client for flavor endpoints.
    #[cfg(feature = "resources")]
    pub flavor: FlavorApi,
    /// Client for flavor group endpoints.
    #[cfg(feature = "resources")]
    pub flavor_group: FlavorGroupApi,
    /// Client for the usage endpoint.
    #[cfg(feature = "resources")]
    pub usage: UsageApi,
    /// Client for flavor price endpoints.
    #[cfg(feature = "pricing")]
    pub flavor_price: FlavorPriceApi,
    /// Client for flavor quota endpoints.
    #[cfg(feature = "quota")]
    pub flavor_quota: FlavorQuotaApi,
    /// Client for server state endpoints.
    #[cfg(feature = "accounting")]
    pub server_state: ServerStateApi,
    /// Client for server cost endpoint.
    #[cfg(feature = "accounting")]
    pub server_cost: ServerCostApi,
    /// Client for server consumption endpoints.
    #[cfg(feature = "accounting")]
    pub server_consumption: ServerConsumptionApi,
    /// Client for project budget endpoints.
    #[cfg(feature = "budgeting")]
    pub project_budget: ProjectBudgetApi,
    /// Client for user budget endpoints.
    #[cfg(feature = "budgeting")]
    pub user_budget: UserBudgetApi,
    /// Client for the budget-over-tree endpoint.
    #[cfg(feature = "budgeting")]
    pub budget_over_tree: BudgetOverTreeApi,
    /// Client for the budget-bulk-create endpoint.
    #[cfg(feature = "budgeting")]
    pub budget_bulk_create: BudgetBulkCreateApi,
}

impl Api {
    /// Create a new API client for the given API URL and with the given authentication token.
    ///
    /// Optionally, the request timeout and a user ID for impersonation may be configured.
    /// The latter is only allowed for admin users.
    pub fn new(
        // TODO: this should be a url::Url
        url: String,
        token: Token,
        impersonate: Option<u32>,
        timeout: Option<u64>,
    ) -> Result<Api, ApiError> {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "X-Auth-Token",
            HeaderValue::from_str(token.as_ref())
                .context("Failed to create token header value")?,
        );
        if let Some(impersonate) = impersonate {
            headers.insert(
                "X-Impersonate",
                HeaderValue::from_str(format!("{impersonate}").as_str())
                    .context("Failed to create impersonate header value")?,
            );
        }
        let timeout = match timeout {
            Some(timeout) => timeout,
            None => DEFAULT_TIMEOUT,
        };
        let mut builder = ClientBuilder::new();

        #[cfg(not(target_family = "wasm"))]
        {
            builder = builder.timeout(Duration::from_secs(timeout));
        }

        let client = Rc::new(
            builder
                .default_headers(headers)
                .build()
                .context("Failed to build http client")?,
        );
        Ok(Api {
            token,
            #[cfg(feature = "hello")]
            hello: HelloApi::new(&url, &client),
            #[cfg(feature = "user")]
            project: ProjectApi::new(&url, &client),
            #[cfg(feature = "user")]
            user: UserApi::new(&url, &client),
            #[cfg(feature = "resources")]
            flavor: FlavorApi::new(&url, &client),
            #[cfg(feature = "resources")]
            flavor_group: FlavorGroupApi::new(&url, &client),
            #[cfg(feature = "resources")]
            usage: UsageApi::new(&url, &client),
            #[cfg(feature = "pricing")]
            flavor_price: FlavorPriceApi::new(&url, &client),
            #[cfg(feature = "quota")]
            flavor_quota: FlavorQuotaApi::new(&url, &client),
            #[cfg(feature = "accounting")]
            server_state: ServerStateApi::new(&url, &client),
            #[cfg(feature = "accounting")]
            server_cost: ServerCostApi::new(&url, &client),
            #[cfg(feature = "accounting")]
            server_consumption: ServerConsumptionApi::new(&url, &client),
            #[cfg(feature = "budgeting")]
            project_budget: ProjectBudgetApi::new(&url, &client),
            #[cfg(feature = "budgeting")]
            user_budget: UserBudgetApi::new(&url, &client),
            #[cfg(feature = "budgeting")]
            budget_over_tree: BudgetOverTreeApi::new(&url, &client),
            #[cfg(feature = "budgeting")]
            budget_bulk_create: BudgetBulkCreateApi::new(&url, &client),
        })
    }
}
