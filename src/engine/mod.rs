mod alpha_beta;
mod evaluate_game;
mod position_scores;
mod random;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::alpha_beta::{alpha_beta, AlphaBetaParams};

pub fn generate_move(game: &Game, color: Color) -> Option<ChessMove> {
    // right now we'll implement a basic strategy of just picking a random move
    // if we can take, we must first take
    // random_move(game, color)
    alpha_beta(game, color, &AlphaBetaParams::default())
}
