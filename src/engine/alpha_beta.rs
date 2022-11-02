use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::evaluate_game::evaluate;

/// Implements the min max algorithm (without alpha beta pruning for now) to decide the best move
/// to play. White is maximizing, black is minimizing.
pub fn alpha_beta(game: &Game, color: Color) -> Option<ChessMove> {
    let max_depth = 4;
    let reasonable_depth = 6;

    let mut best_move = None;

    let mut best_score = f64::NEG_INFINITY;

    let all_valid_moves = game.all_valid_moves_for_color(color);
    let valid_moves_len = all_valid_moves.len();

    if all_valid_moves.is_empty() {
        return None;
    }

    let mut ind = 1;
    for chess_move in all_valid_moves {
        println!("Trying move {} of {}", ind, valid_moves_len);
        let mut new_game = game.clone();
        new_game.make_move(&chess_move);

        let score = alpha_beta_impl(
            &mut new_game,
            color.opposite(),
            f64::NEG_INFINITY,
            f64::INFINITY,
            reasonable_depth + (ind % 2),
            max_depth,
        );

        // new_game.unmake_move(&chess_move);

        if score > best_score {
            best_score = score;
            best_move = Some(chess_move);
        }

        ind += 1;
    }

    best_move
}

fn alpha_beta_impl(game: &mut Game, color: Color, alpha: f64, beta: f64, curr_depth: i32, max_depth: i32) -> f64 {
    if curr_depth <= 0 || max_depth <= 0 {
        return evaluate(game);
    }

    let mut best_eval: f64 = f64::NEG_INFINITY;
    if color == Color::Black {
        best_eval = f64::INFINITY;
    }

    let all_valid_moves = game.all_valid_moves_for_color(color);
    let valid_moves_len = all_valid_moves.len();

    let mut curr_alpha = alpha;
    let mut curr_beta = beta;

    for move_option in all_valid_moves {
        let mut new_game = game.clone();
        new_game.make_move(&move_option);

        let new_curr_depth = if valid_moves_len <= 3 { curr_depth - 1 } else { curr_depth - 2 };

        let eval = alpha_beta_impl(
            &mut new_game,
            color.opposite(),
            curr_alpha,
            curr_beta,
            new_curr_depth,
            max_depth - 1,
        );

        // game.unmake_move(&move_option);

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
