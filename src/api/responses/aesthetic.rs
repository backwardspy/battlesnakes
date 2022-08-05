use serde::Serialize;

use crate::aesthetic::{Colour, Head, Tail};

#[derive(Serialize)]
pub struct AestheticResponse {
    pub author: String,
    pub colour: Colour,
    pub head:   Head,
    pub tail:   Tail,
}

impl Default for AestheticResponse {
    fn default() -> Self {
        let author = env!("CARGO_PKG_AUTHORS")
            .split(':')
            .collect::<Vec<_>>()
            .join(", ");

        Self {
            author,
            colour: Default::default(),
            head: Default::default(),
            tail: Default::default(),
        }
    }
}
