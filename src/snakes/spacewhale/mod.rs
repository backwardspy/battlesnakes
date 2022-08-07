use battlesnake_game_types::wire_representation::Game;

use crate::{
    snakes::Snake,
    wire_representation::{Aesthetic, Movement},
};

pub struct Spacewhale;

impl Snake for Spacewhale {
    fn aesthetic() -> Aesthetic {
        todo!()
    }

    fn start(game: Game) {
        todo!()
    }

    fn make_move(game: Game) -> Movement {
        todo!()
    }

    fn end(game: Game) {
        todo!()
    }
}
