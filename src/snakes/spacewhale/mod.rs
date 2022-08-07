use battlesnake_game_types::{wire_representation::Game, types::Move};

use crate::{
    snakes::Snake,
    wire_representation::{Aesthetic, Movement},
};

pub struct Spacewhale;

impl Snake for Spacewhale {
    fn aesthetic() -> Aesthetic {
        Default::default()
    }

    fn start(_game: Game) {
    }

    fn make_move(_game: Game) -> Movement {
        Movement::new(Move::Up)
    }

    fn end(_game: Game) {
    }
}
