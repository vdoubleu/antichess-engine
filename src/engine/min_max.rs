use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::evaluate_game::evaluate;

/// Implements the min max algorithm (without alpha beta pruning for now) to decide the best move
/// to play. White is maximizing, black is minimizing.
pub fn min_max(game: &Game, color: Color) -> Option<ChessMove> {
    let max_depth = 3;
    let mut best_move = None;

    let mut best_score = f64::NEG_INFINITY;

    let all_valid_moves = game.all_valid_moves_for_color(color);

    if all_valid_moves.len() == 0 {
        return None;
    }

    for chess_move in all_valid_moves {
        let mut game_copy = game.clone();
        game_copy.make_move(&chess_move);

        let score = min_max_impl(&game_copy, max_depth);

        if score > best_score {
            best_score = score;
            best_move = Some(chess_move);
        }
    }

    best_move
}

fn min_max_impl(game: &Game, depth: u32) -> f64 {
    let curr_turn = game.get_player_turn();

    if depth == 0 {
        return evaluate(game, curr_turn);
    }

    let mut best_score: f64 = f64::NEG_INFINITY;
    if curr_turn == Color::Black { 
        best_score = f64::INFINITY;
    } 

    let all_valid_moves = game.all_valid_moves_for_color(curr_turn);

    for move_option in all_valid_moves {
        let mut new_game = game.clone();
        new_game.make_move(&move_option);

        let res = min_max_impl(&new_game, depth - 1);

        if curr_turn == Color::White {
            best_score = res.max(best_score);
        } else {
            best_score = res.min(best_score);
        }
    }

    best_score
}
