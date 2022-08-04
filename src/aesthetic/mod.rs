use serde::Serialize;

use self::{colour::Colour, head::Head, tail::Tail};

pub(super) mod colour;
pub(super) mod head;
pub(super) mod snakeinfo;
pub(super) mod tail;

#[derive(Serialize)]
pub struct Aesthetic {
    pub author: String,
    pub colour: Colour,
    pub head: Head,
    pub tail: Tail,
}

impl Default for Aesthetic {
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
