use serde::Serialize;

use crate::aesthetic::{Head, Tail};

#[derive(Serialize)]
pub struct Aesthetic {
    pub apiversion: String,
    pub author:     String,
    pub color:      String,
    pub head:       Head,
    pub tail:       Tail,
    pub version:    String,
}

impl Default for Aesthetic {
    fn default() -> Self {
        Self {
            apiversion: "1".to_owned(),
            author:     env!("CARGO_PKG_AUTHORS")
                .split(':')
                .collect::<Vec<_>>()
                .join(", "),
            color:      "#888888".to_owned(),
            head:       Default::default(),
            tail:       Default::default(),
            version:    env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}
