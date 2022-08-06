mod colour;
pub use colour::Colour;

mod head;
pub use head::Head;

mod tail;
use serde::Serialize;
pub use tail::Tail;

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

pub trait HasAesthetic {
    fn aesthetic() -> Aesthetic;
}
