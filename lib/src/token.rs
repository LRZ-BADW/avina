use std::{convert::AsRef, str::FromStr};

use anyhow::Context;
use jzon::object;
use reqwest::{
    Client, ClientBuilder,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};

#[derive(Clone, Debug)]
struct TokenInner {
    url: String,
    client: Client,
}

#[derive(Debug)]
pub struct Token {
    token: String,
    inner: Option<TokenInner>,
}

impl Token {
    // TODO: maybe use generic request method in here
    pub async fn new(
        auth_url: &str,
        username: &str,
        password: &str,
        project_name: &str,
        user_domain_name: &str,
        project_domain_id: &str,
    ) -> Result<Self, anyhow::Error> {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        let url = format!("{auth_url}/auth/tokens/");
        let data = object! {
            "auth": {
                "identity": {
                    "methods": ["password"],
                    "password": {
                        "user": {
                            "name": username,
                            "domain": {"name": user_domain_name},
                            "password": password,
                        }
                    }
                },
                "scope": {
                    "project": {
                        "name": project_name,
                        "domain": {"id": project_domain_id}
                    }
                }
            }
        };
        let response = match client
            .post(url.as_str())
            .body(data.to_string())
            .send()
            .await
            .context("")
        {
            Ok(response) => response,
            Err(error) => {
                return Err(anyhow::anyhow!(
                    "Could not complete authentication request: {}",
                    error.root_cause()
                ));
            }
        };
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to authenticate, returned code {}",
                response.status().as_u16()
            ));
        }
        let token = match response.headers().get("X-Subject-Token") {
            Some(token) => token.to_str().unwrap().to_string(),
            None => {
                return Err(anyhow::anyhow!(
                    "No token in authentication response header"
                ));
            }
        }
        .trim()
        .to_string();
        Ok(Self {
            token,
            inner: Some(TokenInner { client, url }),
        })
    }

    pub async fn delete(self) {
        if let Some(inner) = self.inner.clone() {
            let value = HeaderValue::from_str(self.token.as_str()).unwrap();
            inner
                .client
                .delete(inner.url)
                .header("X-Auth-Token", value.clone())
                .header("X-Subject-Token", value)
                .send()
                .await
                .unwrap();
        }
    }
}

impl FromStr for Token {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            // TODO: validate that string has correct format
            token: s.trim().to_string(),
            inner: None,
        })
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        self.token.as_str()
    }
}
