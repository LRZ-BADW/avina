//! Types and functions for accessing avina-ldap.
//!
//! avina-ldap is a helper service, that collects and caches
//! all relevant information from LRZ's LDAP and provides them
//! in easy to deserialize JSON responses.

use std::collections::HashMap;

use actix_web::http::StatusCode;
use anyhow::{Context, anyhow};
use avina_wire::user::UserClass;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

use crate::{error::UnexpectedOnlyError, startup::AvinaLdapConfig};

/// Relevant information for an LRZ user.
#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapUser {
    /// Identifier of the user ("LRZ Kennung").
    name: String,
    /// Identifier of the project ("LRZ Projektkennung").
    project: Option<String>,
    /// Whether the user is master user of their project.
    master: bool,
    /// Whether this is a function account.
    function: bool,
}

/// Relevant information for an LRZ project.
#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapProject {
    /// Identifier of the project ("LRZ Projektkennung").
    name: String,
    /// User class of the institution of the project ("Nutzerklasse").
    class: u32,
}

/// Data of an avina-ldap response.
#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapData {
    /// Map from user identifiers ("LRZ Kennung") to users.
    users: HashMap<String, AvinaLdapUser>,
    /// Map from project identifiers ("LRZ Projektkennung") to projects.
    projects: HashMap<String, AvinaLdapProject>,
    /// Timestamp of the last query to the LRZ LDAP.
    timestamp: DateTime<Utc>,
}

/// Response of avina-ldap.
///
/// In case avina-ldap was just started, it response with
/// nothing, therefore the `Option<AvinaLdapData>`.
#[derive(PartialEq, Clone, Debug, Deserialize)]
struct AvinaLdapResponse(Option<AvinaLdapData>);

/// Abstraction of the avina-ldap service withing this API backend.
///
/// It retrieves and saves the data on initialization and then
/// makes them accessible via member functions.
pub struct AvinaLdap {
    /// Data, that avina-ldap responded with.
    ///
    /// In case avina-ldap was just started, it response with
    /// nothing, therefore the `Option<AvinaLdapData>`.
    data: Option<AvinaLdapData>,
}

impl AvinaLdap {
    /// Call avina-ldap and store the data in an instance of AvinaLdap.
    ///
    /// This assumes that either is avina-ldap configured with URL and token, or that
    /// at the very least, is using defaults allowed. Otherwise a runtime error is
    /// thrown. In case avina-ldap is enabled, this calls it and stores the
    /// received data in the returned instance of [AvinaLdap].
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

    /// Get the userclass of the project with given name, or return the default.
    ///
    /// When there is no data, or the project is not found in it, the [UserClass::NA]
    /// is returned, unlike [AvinaLdap::get_userclass_no_default] which returns the
    /// data in an [Option].
    pub fn get_userclass(&self, project_name: &str) -> UserClass {
        // TODO: are these defaults the best way to handle this
        if let Some(data) = &self.data
            && let Some(project) = data.projects.get(project_name)
        {
            return UserClass::try_from(project.class).unwrap_or(UserClass::NA);
        }
        UserClass::NA
    }

    /// Get the role of the user with given name, or return the default.
    ///
    /// When there is no data, or the user is not found in it, role 1 (normal user)
    /// is returned, unlike [AvinaLdap::get_role_no_default] which returns the data
    /// in an [Option].
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

    /// Get the user class of the project with the given name.
    ///
    /// This returns an [Option] and thus [None] when the data in question
    /// is unavailable, unlike [AvinaLdap::get_userclass] which uses default instead.
    pub fn get_userclass_no_default(
        &self,
        project_name: &str,
    ) -> Option<UserClass> {
        if let Some(data) = &self.data
            && let Some(project) = data.projects.get(project_name)
        {
            return UserClass::try_from(project.class).ok();
        }
        None
    }

    /// Get the role of the user with the given name.
    ///
    /// This returns an [Option] and thus [None] when the data in question
    /// is unavailable, unlike [AvinaLdap::get_role] which uses default instead.
    pub fn get_role_no_default(&self, username: &str) -> Option<u32> {
        if let Some(data) = &self.data
            && let Some(user) = data.users.get(username)
        {
            return Some(if user.master && !user.function { 2 } else { 1 });
        }
        None
    }
}
