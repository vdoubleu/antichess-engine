mod evaluate_game;
mod min_max;
mod random;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::min_max::min_max;

pub fn generate_move(game: &Game, color: Color) -> Option<ChessMove> {
    // right now we'll implement a basic strategy of just picking a random move
    // if we can take, we must first take
    // random_move(game, color)
    min_max(game, color)
}
