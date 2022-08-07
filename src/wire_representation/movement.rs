use battlesnake_game_types::types::Move;
use serde::Serialize;

#[derive(Serialize)]
pub struct Movement {
    direction: Move,
    shout:     Option<String>,
}

impl Movement {
    pub fn new(direction: Move) -> Self {
        Self {
            direction,
            shout: None,
        }
    }

    pub fn with_shout(self, shout: String) -> Self {
        Self {
            direction: self.direction,
            shout:     Some(shout),
        }
    }
}
