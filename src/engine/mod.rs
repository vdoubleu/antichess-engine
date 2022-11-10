mod alpha_beta;
mod evaluate_game;
mod move_sort;
mod position_scores;
mod random;
pub mod store;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::alpha_beta::{alpha_beta, AlphaBetaParams};
use crate::engine::store::AlphaBetaStore;

pub fn generate_move(game: &Game, store: &mut AlphaBetaStore, color: Color) -> Option<ChessMove> {
    // right now we'll implement a basic strategy of just picking a random move
    // if we can take, we must first take
    // random_move(game, color)

    let mut params = AlphaBetaParams::default();

    let target_final_depth = params.depth;

    let mut best_move = None;

    let mut best_score = f64::NEG_INFINITY;

    store.start_turn();

    for curr_depth in 1..target_final_depth {
        if params.debug_print > 0 {
            eprintln!("Starting depth {}", curr_depth);
        }

        if let Some(start_time) = store.start_time {
            if start_time.elapsed() > params.max_time {
                break;
            }
        } else {
            panic!("start time not set");
        }

        params.depth = curr_depth;
        if let Some(res) = alpha_beta(game, color, &params, store) {
            best_move = Some(res.0);
            best_score = res.1;
        }

        if best_score == f64::INFINITY || best_score == f64::NEG_INFINITY {
            break;
        }
    }

    if params.debug_print > 0 {
        eprintln!(
            "search time: {}",
            store.start_time.unwrap().elapsed().as_millis()
        );
    }

    store.end_turn();

    best_move
}
