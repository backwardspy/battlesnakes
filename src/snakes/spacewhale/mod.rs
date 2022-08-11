use battlesnake_game_types::{
    compact_representation::standard::{BestCellBoard, ToBestCellBoard},
    types::{
        FoodGettableGame,
        HazardQueryableGame,
        HeadGettableGame,
        HealthGettableGame,
        LengthGettableGame,
        Move,
        NeighborDeterminableGame,
        RandomReasonableMovesGame,
        SimulableGame,
        SimulatorInstruments,
        SnakeIDGettableGame,
        VictorDeterminableGame,
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

#[derive(Debug)]
struct Instruments {}
impl SimulatorInstruments for Instruments {
    fn observe_simulation(&self, _duration: std::time::Duration) {}
}

fn score<B>(board: &B, new_head: &B::NativePositionType) -> i32
where
    B: FoodGettableGame
        + HeadGettableGame
        + SnakeIDGettableGame
        + YouDeterminableGame
        + NeighborDeterminableGame
        + VictorDeterminableGame
        + LengthGettableGame
        + HealthGettableGame,
{
    let me = board.you_id();

    if let Some(winner) = board.get_winner() {
        if winner == *me {
            return 1000000;
        } else {
            return -1000000;
        }
    }

    let other_sids = board
        .get_snake_ids()
        .into_iter()
        .filter(|sid| sid != me)
        .collect::<Vec<_>>();

    let new_head_pos = board.position_from_native(new_head.clone()).to_vector();
    let dist_to_food = board
        .get_all_food_as_positions()
        .iter()
        .map(|food| food.sub_vec(new_head_pos).manhattan_length() as i32)
        .min()
        .unwrap_or(0);

    let me_length = board.get_length_i64(me);

    let length_diff = (other_sids
        .iter()
        .map(|sid| board.get_length_i64(sid))
        .max()
        .unwrap_or(me_length)
        - me_length) as i32;

    let next_to_scary_snake = other_sids
        .iter()
        .filter(|sid| board.get_length_i64(sid) >= me_length)
        .map(|sid| board.get_head_as_native_position(sid))
        .any(|head| board.neighbors(new_head).any(|pos| head == pos));

    let next_to_baby_snake = other_sids
        .iter()
        .filter(|sid| board.get_length_i64(sid) < me_length)
        .map(|sid| board.get_head_as_native_position(sid))
        .any(|head| board.neighbors(new_head).any(|pos| head == pos));

    -dist_to_food - length_diff * 100 + board.get_health_i64(me) as i32
        - (if next_to_scary_snake { 1000 } else { 0 })
        + (if next_to_baby_snake { 1000 } else { 0 })
}

fn make_move<B, const N_SNAKES: usize>(board: B) -> Movement
where
    B: HeadGettableGame
        + YouDeterminableGame
        + HazardQueryableGame
        + NeighborDeterminableGame
        + FoodGettableGame
        + SimulableGame<Instruments, N_SNAKES>
        + RandomReasonableMovesGame
        + VictorDeterminableGame
        + LengthGettableGame
        + HealthGettableGame,
{
    let you = board.you_id();
    let head = board.get_head_as_native_position(you);

    let moves = [(you.clone(), Move::all())];
    let result = board
        .simulate_with_moves(&Instruments {}, moves.into_iter())
        .collect::<Vec<_>>();

    Movement::new(
        result
            .iter()
            .max_by_key(|(action, board)| {
                let new_head = board
                    .position_from_native(head.clone())
                    .add_vec(action.own_move().to_vector());
                score(board, &board.native_from_position(new_head))
            })
            .map(|(action, _board)| action.own_move())
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
