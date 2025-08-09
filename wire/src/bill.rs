use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[cfg(feature = "tabled")]
use tabled::Tabled;

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Bill {
    pub amount: f64,
    #[tabled(skip)]
    pub pdf: Vec<u8>,
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.amount.to_string())
    }
}
