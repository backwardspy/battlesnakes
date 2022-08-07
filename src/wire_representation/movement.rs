use serde::Serialize;

#[derive(Serialize)]
pub struct Movement {
    direction: battlesnake_game_types::types::Move,
    shout:     Option<String>,
}
