//! Types and functions for accessing OpenStack.
//!
//! The [OpenStack] struct exposes functions for several OpenStack endpoints.
//! The module also contains the types for those requests and responses.
//! The struct also handles token renewal on-the-fly.
//!
//! As `avina-api` only uses few OpenStack endpoints, yet, this module
//! should suffice for now, but later on something like the
//! [openstack](https://docs.rs/openstack/latest/openstack/) crate.
//!
//! For development, these and other the OpenStack API specifications might be useful:
//! * [Nova Compute API](https://docs.openstack.org/api-ref/compute/)
//! * [Keystone Identity API](https://docs.openstack.org/api-ref/compute/)
//!
//! Note, that except from most other modules, in this one "user" and "project"
//! are meant not in the sense of "LRZ users" and "LRZ projects", but OpenStack
//! project and projects, where an "OpenStack user/project" is roughly equivalent to an
//! "LRZ user", and an "OpenStack domain" is roughly equivalent to an "LRZ project".

use std::{collections::HashMap, time::Instant};

use anyhow::Context;
use jzon::object;
use reqwest::{
    ClientBuilder,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::configuration::OpenStackSettings;

/// Wrapper of an OpenStack Keystone token.
///
/// This is used by [OpenStack] for the OpenStack admin token.
struct Token {
    /// Settings needed for accessing the OpenStack services.
    settings: OpenStackSettings,
    /// The actual OpenStack Keystone token.
    token: String,
    /// Timestamp of the last renewal of the token.
    renewed_at: Instant,
}

impl Token {
    /// Issue and wrap a new Keystone token for the configured [OpenStackSettings].
    ///
    /// This already issues a new token with [issue_token] and initializes the timestamp for renewal
    /// to the current time.
    async fn new(settings: &OpenStackSettings) -> Result<Self, anyhow::Error> {
        Ok(Self {
            settings: settings.clone(),
            token: issue_token(settings).await?,
            renewed_at: Instant::now(),
        })
    }

    /// Renew the wrapped Keystone token.
    ///
    /// This reuses [issue_token] and updates the timestamp.
    async fn renew(&mut self) -> Result<(), anyhow::Error> {
        self.token = issue_token(&self.settings).await?;
        self.renewed_at = Instant::now();
        Ok(())
    }

    /// Check if the token is expired.
    ///
    /// By default, Keystone issues tokens with a 24 hour validity. To ensure the
    /// token is always fresh, however, this function considers tokens older than
    /// an hour expired.
    fn is_expired(&self) -> bool {
        self.renewed_at.elapsed().as_secs() > 3600
    }

    /// Get the wrapped Keystone token as cloned [String]
    fn get(&self) -> String {
        self.token.clone()
    }
}

/// Wrapper around [Token] that abstracts away token renewal.
struct TokenHandler {
    /// The wrapped [Token].
    token: RwLock<Token>,
}

impl TokenHandler {
    /// Create a new handler from the given [OpenStackSettings].
    async fn new(settings: &OpenStackSettings) -> Result<Self, anyhow::Error> {
        Ok(TokenHandler {
            token: RwLock::new(Token::new(settings).await?),
        })
    }

    /// Get the current token as [String].
    ///
    /// This automatically renews the wrapped [Token] when expired.
    async fn get(&self) -> String {
        if self.token.read().await.is_expired() {
            self.token.write().await.renew().await.unwrap();
        }
        self.token.read().await.get()
    }
}

/// Abstraction of OpenStack, providing functions for certain API endpoints.
///
/// Examples of provided functions are [OpenStack::validate_user_token]
/// and [OpenStack::get_servers].
// TODO: maybe we could also use rust-openstack at some point.
pub struct OpenStack {
    /// Settings needed for accessing the OpenStack services.
    settings: OpenStackSettings,
    /// Auto-renewing Keystone token for the admin user.
    token: TokenHandler,
}

/// Minimal OpenStack API representation of a project.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ProjectMinimal {
    /// OpenStack project UUID (without dashes).
    pub id: String,
    /// OpenStack project name.
    pub name: String,
}

/// OpenStack API representation of a link (URL) to some API resource.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct Link {
    /// API URL to the resource.
    pub href: String,
    /// Type of relation (self, bookmark, alternate).
    pub rel: String,
}

/// Detailed representation of a flavor in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct FlavorDetailed {
    /// Whether the flavor is enabled.
    #[serde(rename = "OS-FLV-DISABLED:disabled")]
    pub disabled: bool,
    /// Minimal disk space required.
    pub disk: u32,
    // TODO: this does not work, why?
    // #[serde(rename = "OS-FLV-EXT-DATA:ephemeral")]
    // pub ephemeral: bool,
    /// Whether the flavor is publicly available.
    #[serde(rename = "os-flavor-access:is_public")]
    pub is_public: bool,
    /// The UUID of the flavor.
    pub id: String,
    /// Links to the flavor.
    pub links: Vec<Link>,
    /// The name of the flavor.
    pub name: String,
    /// The amount of memory.
    pub ram: u32,
    // TODO: this does not work, why?
    // pub swap: u32,
    /// The amount of vCPUs.
    pub vcpus: u32,
    /// Limit of the factor between receive and transmission.
    pub rxtx_factor: f32,
    /// Description of the flavor.
    pub description: Option<String>,
    // TODO: this is a more complicated field.
    // "extra_specs": {}
}

/// A list of detailed flavors in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct FlavorDetailedList {
    /// The list of flavors.
    flavors: Vec<FlavorDetailed>,
}

/// A flavor in a [ServerDetailed] struct in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct ServerDetailedFlavor {
    /// The UUID of the flavor.
    pub id: String,
    /// Links to this flavor.
    pub links: Vec<Link>,
}

/// A security group in a [ServerDetailed] struct in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct ServerDetailedSecurityGroup {
    /// The name of the security group.
    pub name: String,
}

/// An attached volume in a [ServerDetailed] struct in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct ServerDetailedVolumesAttached {
    /// The UUID of the volume.
    pub id: String,
}

/// A network address in a [ServerDetailed] struct in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct ServerDetailedAddress {
    /// IP address version (4 or 6).
    pub version: usize,
    /// The IP address.
    pub addr: String,
    /// Type of address (fixed or floating).
    #[serde(rename = "OS-EXT-IPS:type")]
    pub addr_type: String,
    /// The MAC address of the port, the address is attached to.
    #[serde(rename = "OS-EXT-IPS-MAC:mac_addr")]
    pub mac_addr: String,
}

/// An image in a [ServerDetailed] struct in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
#[serde(untagged)]
pub enum ServerDetailedImage {
    /// The image ID with links.
    Some {
        /// The image ID.
        id: String,
        /// Links to the image object.
        links: Vec<Link>,
    },
    /// Only the image ID.
    None(String),
}

/// A detailed server representation in the OpenStack API.
// TODO: there are many missing fields here.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct ServerDetailed {
    /// UUID of the server.
    pub id: Uuid,
    /// Name of the server.
    pub name: String,
    /// Description of the server.
    pub description: Option<String>,
    /// Status of the server.
    pub status: String,
    /// UUID of the project owning the server.
    pub tenant_id: String,
    /// UUID of the user owning the server.
    pub user_id: String,
    /// Metadata map.
    pub metadata: HashMap<String, String>,
    /// UUID of the host running this server.
    #[serde(rename = "hostId")]
    pub host_id: String,
    /// Image the server was built from.
    pub image: ServerDetailedImage,
    /// The flavor of the server.
    pub flavor: ServerDetailedFlavor,
    /// Timestamp of creation of the server.
    // TODO: this is actually a datetime
    pub created: String,
    /// Timestamp of the last update of the server.
    // TODO: this is actually a datetime
    pub updated: String,
    /// Network addresses of the server.
    pub addresses: HashMap<String, Vec<ServerDetailedAddress>>,
    /// IPv4 address for accessing the server.
    #[serde(rename = "accessIPv4")]
    pub access_ipv4: String,
    /// IPv6 address for accessing the server.
    #[serde(rename = "accessIPv6")]
    pub access_ipv6: String,
    /// Links to this object.
    pub links: Vec<Link>,
    /// Disk configuration.
    #[serde(rename = "OS-DCF:diskConfig")]
    pub disk_config: String,
    /// Availability zone the server is running in.
    #[serde(rename = "OS-EXT-AZ:availability_zone")]
    pub availability_zone: String,
    /// Configuration drive.
    pub config_drive: String,
    /// Name of the injected SSH keypair.
    pub key_name: Option<String>,
    /// Timestamp of the launch of the server.
    // TODO: this is actually a datetime
    #[serde(rename = "OS-SRV-USG:launched_at")]
    pub launched_at: Option<String>,
    /// Timestamp of the termination of the server.
    // TODO: this is actually a datetime
    #[serde(rename = "OS-SRV-USG:terminated_at")]
    pub terminated_at: Option<String>,
    /// Name of the host running the server.
    #[serde(rename = "OS-EXT-SRV-ATTR:host")]
    pub host: Option<String>,
    /// Name of the libvirt domain of the server.
    #[serde(rename = "OS-EXT-SRV-ATTR:instance_name")]
    pub instance_name: String,
    /// Hostname of the hypervisor running the server.
    #[serde(rename = "OS-EXT-SRV-ATTR:hypervisor_hostname")]
    pub hypervisor_hostname: Option<String>,
    /// Task state for the server.
    #[serde(rename = "OS-EXT-STS:task_state")]
    pub task_state: Option<String>,
    /// State of the VM.
    #[serde(rename = "OS-EXT-STS:vm_state")]
    pub vm_state: String,
    /// Power state of the server.
    #[serde(rename = "OS-EXT-STS:power_state")]
    pub power_state: usize,
    /// Volumes attached to the server.
    #[serde(rename = "os-extended-volumes:volumes_attached")]
    pub volumes_attached: Vec<ServerDetailedVolumesAttached>,
    /// Security groups used for the server.
    pub security_groups: Option<Vec<ServerDetailedSecurityGroup>>,
}

/// A detailed list of servers in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ServerDetailedList {
    /// The list of servers.
    servers: Vec<ServerDetailed>,
}

/// Representation of a project domain in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct Domain {
    /// UUID of the domain.
    pub id: String,
    /// Name of the domain.
    pub name: String,
    /// Description of the domain.
    pub description: Option<String>,
    /// Whether the domain is enabled.
    pub enabled: bool,
}

/// A list of project domains in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct DomainList {
    /// The list of domains.
    domains: Vec<Domain>,
}

/// Representation of a project in the OpenStack API.
// TODO: there are fields missing here
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct Project {
    /// UUID of the project.
    pub id: String,
    /// Name of the project.
    pub name: String,
    /// Description of the project.
    pub description: Option<String>,
    /// Whether the project is enabled.
    pub enabled: bool,
    /// Whether the project is a domain.
    pub is_domain: bool,
    /// The UUID of the domain the project belongs to.
    pub domain_id: String,
    /// The UUID of the parent project.
    pub parent_id: String,
    /// Tags of the project.
    pub tags: Vec<String>,
}

/// A list of projects in the OpenStack API.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ProjectList {
    /// The project list.
    projects: Vec<Project>,
}

impl OpenStack {
    /// Create a new instance of the OpenStack abstraction.
    ///
    /// This merely copies the settings and creates the token handler,
    /// which already issues the initial token.
    pub async fn new(
        settings: OpenStackSettings,
    ) -> Result<Self, anyhow::Error> {
        Ok(OpenStack {
            token: TokenHandler::new(&settings).await?,
            settings,
        })
    }

    /// Setup a HTTP client with the token in the default headers.
    async fn client(&self) -> Result<reqwest::Client, anyhow::Error> {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "X-Auth-Token",
            HeaderValue::from_str(self.token.get().await.as_str())
                .context("Could not create token header")?,
        );
        ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context("Could not create client")
    }

    /// Validate a given user token against OpenStack Keystone.
    ///
    /// The user token is the one to be validated and given to Keystone as "subject token" also via
    /// a header, and needs to be distinguished from the admin token authenticating the validation
    /// request. When the validation succeeds the project the token belongs to is returned.
    pub async fn validate_user_token(
        &self,
        token: &str,
    ) -> Result<ProjectMinimal, anyhow::Error> {
        #[derive(Debug, serde::Deserialize)]
        struct ValidateResponseToken {
            project: ProjectMinimal,
        }
        #[derive(Debug, serde::Deserialize)]
        struct ValidateResponse {
            token: ValidateResponseToken,
        }

        let client = self.client().await?;
        let url = format!("{}/auth/tokens/", self.settings.keystone_endpoint);
        let response = client
            .get(url.as_str())
            .header("X-Subject-Token", token)
            .send()
            .await
            .context("Could not validate user token")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to validate user token, returned code {}",
                response.status().as_u16()
            ));
        }
        let project: ValidateResponse = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read response text")?
                .as_str(),
        )
        .context("Could not parse response")?;
        Ok(project.token.project)
    }

    /// Get a list of all flavors in detailed representation.
    pub async fn get_flavors(
        &self,
    ) -> Result<Vec<FlavorDetailed>, anyhow::Error> {
        let client = self.client().await?;
        let url = format!(
            "{}/v2.1/flavors/detail?is_public=False",
            self.settings.nova_endpoint
        );
        let response = client
            .get(url.as_str())
            .send()
            .await
            .context("Could not retrieve flavor list")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to validate user token, returned code {}",
                response.status().as_u16()
            ));
        }
        let flavors: FlavorDetailedList = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read response text")?
                .as_str(),
        )
        .context("Could not parse response")?;
        Ok(flavors.flavors)
    }

    /// Get a list of all servers in detailed representation.
    pub async fn get_servers(
        &self,
    ) -> Result<Vec<ServerDetailed>, anyhow::Error> {
        let client = self.client().await?;
        let url = format!(
            "{}/v2.1/servers/detail?all_tenants=True",
            self.settings.nova_endpoint
        );
        let response = client
            .get(url.as_str())
            .send()
            .await
            .context("Could not retrieve server list")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to validate user token, returned code {}",
                response.status().as_u16()
            ));
        }
        let servers: ServerDetailedList = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read response text")?
                .as_str(),
        )
        .context("Could not parse response")?;
        Ok(servers.servers)
    }

    /// Get a list of servers in detailed representation for the given project.
    pub async fn get_servers_of_project(
        &self,
        project_id: String,
    ) -> Result<Vec<ServerDetailed>, anyhow::Error> {
        let client = self.client().await?;
        let url = format!(
            "{}/v2.1/servers/detail?all_tenants=True&tenant_id={}",
            self.settings.nova_endpoint, project_id,
        );
        let response = client
            .get(url.as_str())
            .send()
            .await
            .context("Could not retrieve server list")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to validate user token, returned code {}",
                response.status().as_u16()
            ));
        }
        let servers: ServerDetailedList = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read response text")?
                .as_str(),
        )
        .context("Could not parse response")?;
        Ok(servers.servers)
    }

    /// Get a list of all domains (e.g., LRZ projects.)
    pub async fn get_domains(&self) -> Result<Vec<Domain>, anyhow::Error> {
        let client = self.client().await?;
        let url = format!("{}/domains", self.settings.keystone_endpoint);
        let response = client
            .get(url.as_str())
            .send()
            .await
            .context("Could not retrieve domain list")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to validate user token, returned code {}",
                response.status().as_u16()
            ));
        }
        let domains: DomainList = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read response text")?
                .as_str(),
        )
        .context("Could not parse response")?;
        Ok(domains.domains)
    }

    /// Get a list of all projects (e.g., LRZ users).
    pub async fn get_projects(&self) -> Result<Vec<Project>, anyhow::Error> {
        let client = self.client().await?;
        let url = format!("{}/projects", self.settings.keystone_endpoint);
        let response = client
            .get(url.as_str())
            .send()
            .await
            .context("Could not retrieve project list")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to validate user token, returned code {}",
                response.status().as_u16()
            ));
        }
        let projects: ProjectList = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read response text")?
                .as_str(),
        )
        .context("Could not parse response")?;
        Ok(projects.projects)
    }
}

/// Issue a new authentication token from the given [OpenStackSettings].
///
/// This function is kept separate from the [OpenStack] implementation, as it
/// is necessary for its creation.
#[tracing::instrument(name = "Issue an OpenStack token", skip(settings))]
pub async fn issue_token(
    settings: &OpenStackSettings,
) -> Result<String, anyhow::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();
    let url = format!("{}/auth/tokens/", settings.keystone_endpoint);
    let data = object! {
        "auth": {
            "identity": {
                "methods": ["password"],
                "password": {
                    "user": {
                        "name": settings.username.clone(),
                        "domain": {"name": settings.domain.clone()},
                        "password": settings.password.clone(),
                    }
                }
            },
            "scope": {
                "project": {
                    "name": settings.project.clone(),
                    "domain": {"id": settings.domain_id.clone()}
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
    Ok(token)
}
