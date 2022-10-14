use crate::chess_game::{Color, Game, ChessMove};
use rand::prelude::SliceRandom;

pub fn generate_move(game: &Game, color: Color) -> Option<ChessMove> {
    // right now we'll implement a basic strategy of just picking a random move
    let all_valid_moves = game.all_valid_moves_for_color(color);

    let mut rng = rand::thread_rng();
    
    match all_valid_moves.choose(&mut rng) {
        Some(m) => return Some(*m),
        None => return None,
    }
}