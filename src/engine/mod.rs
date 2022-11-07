mod alpha_beta;
mod evaluate_game;
mod move_sort;
mod position_scores;
mod random;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::alpha_beta::{alpha_beta, AlphaBetaParams, AlphaBetaStore};

pub fn generate_move(game: &Game, color: Color) -> Option<ChessMove> {
    // right now we'll implement a basic strategy of just picking a random move
    // if we can take, we must first take
    // random_move(game, color)

    let mut params = AlphaBetaParams::default();

    let target_final_depth = params.depth;

    let mut best_move = None;

    let mut store = AlphaBetaStore::new();

    let mut best_score = f64::NEG_INFINITY;

    for curr_depth in 1..target_final_depth {
        if store.start_time.elapsed() > store.max_time {
            break;
        }

        params.depth = curr_depth;
        if let Some(res) = alpha_beta(game, color, &params, &mut store) {
            best_move = Some(res.0);
            best_score = res.1;
        }

        if best_score == f64::INFINITY || best_score == f64::NEG_INFINITY {
            break;
        }
    }

    best_move
}
