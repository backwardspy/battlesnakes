use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize)]
pub struct MoveResponse {
    direction: Direction,
    shout:     Option<String>,
}

impl MoveResponse {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            shout: None,
        }
    }

    pub fn with_shout(mut self, shout: &str) -> Self {
        self.shout = Some(shout.to_owned());
        self
    }
}
