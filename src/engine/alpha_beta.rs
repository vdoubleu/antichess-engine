use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::evaluate_game::evaluate;

/// Implements the min max algorithm (without alpha beta pruning for now) to decide the best move
/// to play. White is maximizing, black is minimizing.
pub fn alpha_beta(game: &Game, color: Color) -> Option<ChessMove> {
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

        let score = alpha_beta_impl(
            &game_copy,
            color.opposite(),
            f64::NEG_INFINITY,
            f64::INFINITY,
            max_depth,
        );

        if score > best_score {
            best_score = score;
            best_move = Some(chess_move);
        }
    }

    best_move
}

fn alpha_beta_impl(game: &Game, color: Color, alpha: f64, beta: f64, depth: u32) -> f64 {
    if depth == 0 {
        return evaluate(game);
    }

    let mut best_eval: f64 = f64::NEG_INFINITY;
    if color == Color::Black {
        best_eval = f64::INFINITY;
    }

    let mut all_valid_moves: Vec<ChessMove> = game.all_valid_moves_for_color_that_take(color);

    if all_valid_moves.is_empty() {
        all_valid_moves = game.all_valid_moves_for_color(color);
    }

    let mut curr_alpha = alpha;
    let mut curr_beta = beta;

    for move_option in all_valid_moves {
        let mut new_game = game.clone();
        new_game.make_move(&move_option);

        let eval = alpha_beta_impl(
            &new_game,
            color.opposite(),
            curr_alpha,
            curr_beta,
            depth - 1,
        );

        if color == Color::White {
            best_eval = eval.max(best_eval);
            curr_alpha = curr_alpha.max(eval);

            if beta <= alpha {
                break;
            }
        } else {
            best_eval = eval.min(best_eval);
            curr_beta = curr_beta.min(eval);

            if beta <= alpha {
                break;
            }
        }
    }

    best_eval
}

#[cfg(test)]
mod alpha_beta_tests {
    use super::*;
    use crate::chess_game::{Color, Game};

    #[test]
    fn test_alpha_beta() {
        let mut game = Game::new_starting_game();

        let move1 = alpha_beta(&game, Color::White);
        assert!(move1.is_some());

        game.make_move(&move1.unwrap());

        let move2 = alpha_beta(&game, Color::Black);
        assert!(move2.is_some());

        game.make_move(&move2.unwrap());
    }
}
