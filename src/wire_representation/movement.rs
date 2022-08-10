use battlesnake_game_types::types::Move;
use serde::Serialize;

struct LowercaseMove(Move);

impl From<Move> for LowercaseMove {
    fn from(m: Move) -> Self {
        Self(m)
    }
}

impl Serialize for LowercaseMove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Move::Left => "left".serialize(serializer),
            Move::Up => "up".serialize(serializer),
            Move::Down => "down".serialize(serializer),
            Move::Right => "right".serialize(serializer),
        }
    }
}

#[derive(Serialize)]
pub struct Movement {
    #[serde(rename = "move")]
    direction: LowercaseMove,
    shout:     Option<String>,
}

impl Movement {
    pub fn new(direction: Move) -> Self {
        Self {
            direction: LowercaseMove::from(direction),
            shout:     None,
        }
    }

    pub fn with_shout(self, shout: String) -> Self {
        Self {
            direction: self.direction,
            shout:     Some(shout),
        }
    }
}
