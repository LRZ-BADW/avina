//! Types for avina's pricing module.

use std::fmt::Display;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tabled")]
use tabled::Tabled;

use crate::user::UserClass;

/// Price of a flavor per year for a user class.
///
/// Prices are unique with respects to their flavor, user class and start time,
/// meaning at any time, there is at most one valid price for a given flavor and
/// user class. After the [Self::start_time] of the price it is valid, until
/// another price for the same flavor and user class replaces it.
///
/// The [Self::unit_price] gives the amount of EUROs, that a single VM run for an
/// entire year with the [Self::flavor] by a user of a project with the
/// [Self::user_class] adds in cost to the project, within the limits of the
/// project's budget and assuming that the price was valid the entire year.
/// The cost is calculated proportionally if the VM ran less long in this flavor
/// or the price was valid for less long.
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorPrice {
    /// ID of the flavor price.
    pub id: u32,
    /// ID of the flavor.
    pub flavor: u32,
    /// Name of the flavor.
    pub flavor_name: String,
    /// User class the price is for.
    pub user_class: UserClass,
    /// Price in EURO per VM per year.
    pub unit_price: f64,
    /// Time after which the price is valid.
    pub start_time: DateTime<FixedOffset>,
}

impl Display for FlavorPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "FlavorPrice(id={}, flavor={})",
            self.id, self.flavor_name
        ))
    }
}

/// Response from the flavor-price-initialize endpoint.
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlavorPriceInitialize {
    /// Number of newly initialized flavors.
    pub new_flavor_price_count: u32,
}

/// Request data for creating a new flavor price with the flavor-price-create endpoint.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlavorPriceCreateData {
    /// ID of the flavor the price is for.
    pub flavor: u32,
    /// User class the price is for.
    pub user_class: UserClass,
    /// Optional unit price (0. by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Optional start time (now by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<FixedOffset>>,
}

impl FlavorPriceCreateData {
    /// Create a new instance of [FlavorPriceCreateData] from only the mandatory parameters.
    pub fn new(flavor: u32, user_class: UserClass) -> Self {
        Self {
            flavor,
            user_class,
            price: None,
            start_time: None,
        }
    }
}

/// Request data for modifying a flavor price with the flavor-price-modify endpoint.
///
/// All fields by [Self::id] are optional to allow partial modification.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlavorPriceModifyData {
    /// ID of the flavor price to modify.
    pub id: u32,

    /// ID of the flavor the price is for, not changed if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<u32>,
    /// User class the price is for, not changed if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_class: Option<UserClass>,
    /// Unit price per VM per year, not changed if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    /// Start time of the price, not changed if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<FixedOffset>>,
}

impl FlavorPriceModifyData {
    /// Create a new instance of [FlavorPriceModifyData] from only the mandatory parameters.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            flavor: None,
            user_class: None,
            unit_price: None,
            start_time: None,
        }
    }
}

/// Request URL parameters for listing prices with the flavor-price-list endpoint.
///
/// All members are optional, to allow not specifying them.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct FlavorPriceListParams {
    /// Filter by the user class, not filtered by if not set.
    pub user_class: Option<UserClass>,
    /// Filter only current prices, not filtered by if not set.
    pub current: Option<bool>,
}
