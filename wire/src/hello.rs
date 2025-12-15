use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[cfg(feature = "tabled")]
use tabled::Tabled;

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Hello {
    pub message: String,
}

impl Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}
