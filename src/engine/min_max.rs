use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::evaluate_game::evaluate;

/// Implements the min max algorithm (without alpha beta pruning for now) to decide the best move
/// to play. White is maximizing, black is minimizing.
pub fn min_max(game: &Game, color: Color) -> Option<ChessMove> {
    let max_depth = 3;
    let mut best_move = None;

    let mut best_score = f64::NEG_INFINITY;

    let mut all_valid_moves = game.all_valid_moves_for_color_that_take(color);

    if all_valid_moves.is_empty() {
        all_valid_moves = game.all_valid_moves_for_color(color);

        if all_valid_moves.is_empty() {
            return None;
        }
    }

    for chess_move in all_valid_moves {
        let mut game_copy = game.clone();
        game_copy.make_move(&chess_move);

        let score = min_max_impl(&game_copy, color.opposite(), max_depth);

        if score > best_score {
            best_score = score;
            best_move = Some(chess_move);
        }
    }

    best_move
}

fn min_max_impl(game: &Game, color: Color, depth: u32) -> f64 {
    if depth == 0 {
        return evaluate(game);
    }

    let mut best_score: f64 = f64::NEG_INFINITY;
    if color == Color::Black {
        best_score = f64::INFINITY;
    }

    let mut all_valid_moves = game.all_valid_moves_for_color_that_take(color);

    if all_valid_moves.is_empty() {
        all_valid_moves = game.all_valid_moves_for_color(color);
    }

    for move_option in all_valid_moves {
        let mut new_game = game.clone();
        new_game.make_move(&move_option);

        let res = min_max_impl(&new_game, color.opposite(), depth - 1);

        if color == Color::White {
            best_score = res.max(best_score);
        } else {
            best_score = res.min(best_score);
        }
    }

    best_score
}

#[cfg(test)]
mod min_max_tests {
    use super::*;
    use crate::chess_game::{Color, Game};

    #[test]
    fn test_min_max() {
        let mut game = Game::new_starting_game();

        let move1 = min_max(&game, Color::White);
        assert!(move1.is_some());

        game.make_move(&move1.unwrap());

        let move2 = min_max(&game, Color::Black);
        assert!(move2.is_some());

        game.make_move(&move2.unwrap());
    }
}
