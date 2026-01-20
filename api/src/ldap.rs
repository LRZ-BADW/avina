use std::collections::HashMap;

use actix_web::http::StatusCode;
use anyhow::{Context, anyhow};
use avina_wire::user::UserClass;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

use crate::{error::UnexpectedOnlyError, startup::AvinaLdapConfig};

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapUser {
    name: String,
    project: Option<String>,
    master: bool,
    function: bool,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapProject {
    name: String,
    class: u32,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapData {
    users: HashMap<String, AvinaLdapUser>,
    projects: HashMap<String, AvinaLdapProject>,
    timestamp: DateTime<Utc>,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapResponse(Option<AvinaLdapData>);

pub struct AvinaLdap {
    data: Option<AvinaLdapData>,
}

impl AvinaLdap {
    #[tracing::instrument(name = "call_avina_ldap")]
    pub async fn new(
        config: &AvinaLdapConfig,
    ) -> Result<Self, UnexpectedOnlyError> {
        let (url, token, default) = match &config {
            AvinaLdapConfig::Enabled(url, token, default) => {
                (url, token, default)
            }
            AvinaLdapConfig::Disabled(default) => {
                if !default {
                    return Err(anyhow!(
                        "avina-ldap disabled but using defaults also not configured."
                    ).into());
                }
                return Ok(Self { data: None });
            }
        };
        let response = Client::new()
            .get(url)
            .header("Authorization", token)
            .send()
            .await
            .context("Call to avina-ldap failed.")?;
        if response.status().as_u16() != StatusCode::OK {
            return Err(
                anyhow!("avina-ldap returned non-OK status code.").into()
            );
        }
        let data: AvinaLdapResponse = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read avina-ldap response text")?
                .as_str(),
        )
        .context("Could not parse avina-ldap response")?;
        if data.0.is_none() && !default {
            return Err(anyhow!(
                    "avina-ldap returned nothing but using defaults also not configured."
                ).into());
        }
        // TODO: we could also check if the data is outdated.
        Ok(Self { data: data.0 })
    }

    pub fn get_userclass(&self, project_name: &str) -> UserClass {
        // TODO: are these defaults the best way to handle this
        if let Some(data) = &self.data
            && let Some(project) = data.projects.get(project_name)
        {
            return UserClass::try_from(project.class).unwrap_or(UserClass::NA);
        }
        UserClass::NA
    }

    pub fn get_role(&self, username: &str) -> u32 {
        if let Some(data) = &self.data
            && let Some(user) = data.users.get(username)
            && user.master
            && !user.function
        {
            2
        } else {
            1
        }
    }
}
