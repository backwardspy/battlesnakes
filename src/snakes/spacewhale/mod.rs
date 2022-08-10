use battlesnake_game_types::{
    compact_representation::standard::{BestCellBoard, ToBestCellBoard},
    types::{
        FoodGettableGame,
        HazardQueryableGame,
        HeadGettableGame,
        Move,
        NeighborDeterminableGame,
        SnakeBodyGettableGame,
        YouDeterminableGame,
    },
    wire_representation::Game,
};
use color_eyre::{eyre::eyre, Result};

use crate::{
    aesthetic::{Head, Tail},
    snakes::Snake,
    wire_representation::{Aesthetic, Movement},
};

pub struct Spacewhale;

fn score<B>(board: &B, new_head: &B::NativePositionType) -> i32
where
    B: FoodGettableGame,
{
    0
}

fn make_move<B>(board: B) -> Movement
where
    B: HeadGettableGame
        + YouDeterminableGame
        + HazardQueryableGame
        + NeighborDeterminableGame
        + FoodGettableGame,
{
    let you = board.you_id();
    let head = board.get_head_as_native_position(you);

    Movement::new(
        board
            .possible_moves(&head)
            .filter(|(_, new_head)| {
                !board.position_is_snake_body(new_head.clone())
                    && !board.is_hazard(new_head)
            })
            .max_by_key(|(_, new_head)| score(&board, new_head))
            .map(|(direction, _)| direction)
            .unwrap_or(Move::Up),
    )
}

impl Snake for Spacewhale {
    fn aesthetic() -> Aesthetic {
        Aesthetic {
            color: "#89b4fa".to_owned(),
            head: Head::Whale,
            tail: Tail::Comet,
            ..Default::default()
        }
    }

    fn start(_game: Game) {}

    fn make_move(game: Game) -> Result<Movement> {
        let cellboard = game
            .to_best_cell_board()
            .map_err(|_| eyre!("failed to build cellboard for game"))?;

        Ok(match cellboard {
            BestCellBoard::Tiny(board) => make_move(*board),
            BestCellBoard::SmallExact(board) => make_move(*board),
            BestCellBoard::Standard(board) => make_move(*board),
            BestCellBoard::MediumExact(board) => make_move(*board),
            BestCellBoard::LargestU8(board) => make_move(*board),
            BestCellBoard::LargeExact(board) => make_move(*board),
            BestCellBoard::ArcadeMaze(board) => make_move(*board),
            BestCellBoard::Large(board) => make_move(*board),
            BestCellBoard::Silly(board) => make_move(*board),
        })
    }

    fn end(_game: Game) {}
}
