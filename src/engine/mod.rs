mod min_max;
mod move_option_node;
mod evaluate_game;

use crate::chess_game::{ChessMove, Color, Game};
use rand::prelude::SliceRandom;

pub fn generate_move(game: &Game, color: Color) -> Option<ChessMove> {
    // right now we'll implement a basic strategy of just picking a random move
    // if we can take, we must first take
    random_move(game, color)
}

fn random_move(game: &Game, color: Color) -> Option<ChessMove> {
    let all_valid_moves_that_take = game.all_valid_moves_for_color_that_take(color);

    if all_valid_moves_that_take.len() > 0 {
        let mut rng = rand::thread_rng();
        return all_valid_moves_that_take.choose(&mut rng).copied();
    }

    // otherwise, just pick a random move
    let all_valid_moves = game.all_valid_moves_for_color(color);
    let mut rng = rand::thread_rng();
    all_valid_moves.choose(&mut rng).copied()
}

