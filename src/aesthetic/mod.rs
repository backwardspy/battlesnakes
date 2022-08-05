use crate::api::responses::aesthetic::AestheticResponse;

mod colour;
pub use colour::Colour;

mod head;
pub use head::Head;

mod tail;
pub use tail::Tail;

pub trait HasAesthetic {
    fn aesthetic() -> AestheticResponse;
}
