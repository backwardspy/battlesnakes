use crate::{
    aesthetic::HasAesthetic,
    api::responses::aesthetic::AestheticResponse,
};

pub struct Spacewhale;

impl HasAesthetic for Spacewhale {
    fn aesthetic() -> AestheticResponse {
        AestheticResponse {
            ..Default::default()
        }
    }
}
