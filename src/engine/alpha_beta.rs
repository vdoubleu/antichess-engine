use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::evaluate_game::evaluate;

/// Implements the min max algorithm (without alpha beta pruning for now) to decide the best move
/// to play. White is maximizing, black is minimizing.
pub fn alpha_beta(game: &Game, color: Color) -> Option<ChessMove> {
    let max_depth = 16;
    let reasonable_depth = 8;

    let mut best_move = None;

    let mut best_score = if color == Color::White {
        f64::NEG_INFINITY
    } else {
        f64::INFINITY
    };

    let all_valid_moves = game.all_valid_moves_for_color(color);
    let valid_moves_len = all_valid_moves.len();

    if all_valid_moves.is_empty() {
        return None;
    }

    let mut new_game = game.clone();

    let mut ind = 1;
    for chess_move in all_valid_moves {
        eprintln!("Trying move {} of {}", ind, valid_moves_len);
        new_game.move_piece(&chess_move);

        let score = alpha_beta_impl(
            &mut new_game,
            color.opposite(),
            f64::NEG_INFINITY,
            f64::INFINITY,
            reasonable_depth + (ind % 2),
            max_depth,
            true,
        );

        new_game.unmove_move();

        if (color == Color::White && score >= best_score)
            || (color == Color::Black && score <= best_score)
        {
            best_score = score;
            best_move = Some(chess_move);
        }

        ind += 1;
    }

    best_move
}

fn alpha_beta_impl(
    game: &mut Game,
    color: Color,
    alpha: f64,
    beta: f64,
    curr_depth: i32,
    max_depth: i32,
    do_null: bool,
) -> f64 {
    if curr_depth <= 0 || max_depth <= 0 {
        return evaluate(game);
    }

    let mut best_eval: f64 = f64::NEG_INFINITY;
    if color == Color::Black {
        best_eval = f64::INFINITY;
    }

    // do null move pruning if we are at a reasonable depth
    // and the game is not over
    if do_null && game.turn_counter > 0 && curr_depth >= 4 {
        game.move_piece(&ChessMove::new_null_move());

        let eval = alpha_beta_impl(
            game,
            color.opposite(),
            -beta,
            -beta + 1.0,
            curr_depth - 4,
            max_depth - 1,
            false,
        );

        game.unmove_move();

        if eval >= beta && eval.abs() < f64::INFINITY {
            return beta;
        }
    }

    let all_valid_moves = game.all_valid_moves_for_color(color);
    let valid_moves_len = all_valid_moves.len();

    let mut curr_alpha = alpha;
    let mut curr_beta = beta;

    let new_curr_depth = if valid_moves_len <= 3 {
        curr_depth
    } else {
        curr_depth - 2
    };

    for move_option in all_valid_moves {
        // println!("move: {} {} {:?}", move_option, color, move_option);

        game.move_piece(&move_option);

        if let Some(winner) = game.winner {
            game.unmove_move();
            if winner == Color::White {
                return f64::INFINITY;
            } else {
                return f64::NEG_INFINITY;
            }
        }

        let eval = alpha_beta_impl(
            game,
            color.opposite(),
            curr_alpha,
            curr_beta,
            new_curr_depth,
            max_depth - 1,
            true,
        );

        game.unmove_move();

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

        game.move_piece(&move1.unwrap());
    }
}
