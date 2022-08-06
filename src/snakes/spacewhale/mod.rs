use crate::aesthetic::{Aesthetic, HasAesthetic};

pub struct Spacewhale;

impl HasAesthetic for Spacewhale {
    fn aesthetic() -> Aesthetic {
        Aesthetic {
            ..Default::default()
        }
    }
}
