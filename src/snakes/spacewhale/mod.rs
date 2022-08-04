use crate::{Aesthetic, SnakeInfo};

pub struct Spacewhale;

impl SnakeInfo for Spacewhale {
    fn snake_info() -> Aesthetic {
        Aesthetic {
            ..Default::default()
        }
    }
}
