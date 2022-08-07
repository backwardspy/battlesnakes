use battlesnake_game_types::wire_representation::Game;

use crate::wire_representation::{Aesthetic, Movement};

pub trait Snake {
    fn aesthetic() -> Aesthetic;
    fn start(game: Game);
    fn make_move(game: Game) -> Movement;
    fn end(game: Game);
}
