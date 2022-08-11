// clippy bug flags our enums with this lint incorrectly :(
#![allow(clippy::use_self)]

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Head {
    Default,
    Whale,
}

impl Default for Head {
    fn default() -> Self {
        Self::Default
    }
}
