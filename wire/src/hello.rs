//! Types for avina's hello module.

use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[cfg(feature = "tabled")]
use tabled::Tabled;

/// Response from the hello-user and hello-admin endpoints.
#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Hello {
    /// Wrapped message.
    pub message: String,
}

impl Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}
