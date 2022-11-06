use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::evaluate_game::evaluate;

pub struct AlphaBetaParams {
    /// the usual depth to search to.
    /// With forced moves, it will often cause it to go deeper than this value.
    pub depth: i32,
    /// the maximum depth to search. This is the hard limit.
    pub max_depth: i32,
    /// When performing null move pruning, we often don't need to go as deep. This is how much less
    /// deep we go compared to the normal depth.
    pub null_move_reduction: i32,
    /// enables debug printing
    pub debug_print: bool,
}

impl Default for AlphaBetaParams {
    fn default() -> Self {
        AlphaBetaParams {
            depth: 4,
            max_depth: 16,
            null_move_reduction: 2,
            debug_print: false,
        }
    }
}

/// Implements the min max algorithm (without alpha beta pruning for now) to decide the best move
/// to play. White is maximizing, black is minimizing.
pub fn alpha_beta(game: &Game, color: Color, params: &AlphaBetaParams) -> Option<ChessMove> {
    let reasonable_depth = params.depth;
    let max_depth = params.max_depth;

    let mut best_move = None;

    let mut best_score = f64::NEG_INFINITY;

    let all_valid_moves = game.all_valid_moves_for_color(color);
    let valid_moves_len = all_valid_moves.len();

    if all_valid_moves.is_empty() {
        return None;
    }

    let mut new_game = game.clone();

    let mut ind = 1;
    for chess_move in all_valid_moves {
        if params.debug_print {
            eprintln!("Trying move {} of {}", ind, valid_moves_len);
        }

        new_game.move_piece(&chess_move);

        let eval = -alpha_beta_impl(
            &mut new_game,
            f64::NEG_INFINITY,
            f64::INFINITY,
            reasonable_depth + (ind % 2),
            max_depth,
            true,
            params,
        );

        new_game.unmove_move();

        if eval >= best_score {
            best_score = eval;
            best_move = Some(chess_move);
        }

        ind += 1;
    }

    best_move
}

fn alpha_beta_impl(
    game: &mut Game,
    alpha: f64,
    beta: f64,
    curr_depth: i32,
    max_depth: i32,
    do_null: bool,
    params: &AlphaBetaParams,
) -> f64 {
    if curr_depth <= 0 || max_depth <= 0 || game.winner.is_some() {
        let pov = if game.player_turn == Color::White {
            1.0
        } else {
            -1.0
        };

        return pov * evaluate(game);
    }

    let mut score = f64::NEG_INFINITY;

    // do null move pruning if we are at a reasonable depth
    // and the game is not over
    if do_null && game.turn_counter > 0 && curr_depth >= 4 && params.null_move_reduction > 0 {
        game.move_piece(&ChessMove::new_null_move());

        let eval = -alpha_beta_impl(
            game,
            -beta,
            -beta + 1.0,
            curr_depth - params.null_move_reduction,
            max_depth - 1,
            false,
            params,
        );

        game.unmove_move();

        if eval >= beta && eval.abs() < f64::INFINITY {
            return beta;
        }
    }

    let all_valid_moves = game.all_valid_moves_for_color(game.player_turn);
    let valid_moves_len = all_valid_moves.len();

    let mut curr_alpha = alpha;

    let new_curr_depth = if valid_moves_len <= 3 {
        curr_depth
    } else {
        curr_depth - 1
    };

    for move_option in all_valid_moves {
        if params.debug_print {
            println!(
                "move: {} {} {:?}",
                game.player_turn, move_option, move_option
            );
        }

        game.move_piece(&move_option);

        let eval = -alpha_beta_impl(
            game,
            -beta,
            -curr_alpha,
            new_curr_depth,
            max_depth - 1,
            true,
            params,
        );

        game.unmove_move();

        if eval >= score {
            score = eval;

            if eval > curr_alpha {
                if eval >= beta {
                    return beta;
                }
                curr_alpha = eval;
            }
        }
    }

    score
}

#[cfg(test)]
mod alpha_beta_tests {
    use super::*;
    use crate::chess_game::{Color, Game};

    #[test]
    fn test_alpha_beta() {
        let mut game = Game::new_starting_game();

        let move1 = alpha_beta(&game, Color::White, &AlphaBetaParams::default());
        assert!(move1.is_some());

        game.move_piece(&move1.unwrap());
    }

    #[test]
    fn white_makes_better_move() {
        let game = Game::from_fen_notation("7k/8/8/3q1n2/4P3/8/8/7K");

        let move1_option = alpha_beta(&game, Color::White, &AlphaBetaParams::default());
        assert!(move1_option.is_some());

        let move1 = move1_option.unwrap();
        assert_eq!(move1, ChessMove::from_xboard_algebraic_notation("e4d5"));

        // try again with a alpha beta depth of 3
        let move2_option = alpha_beta(
            &game,
            Color::White,
            &AlphaBetaParams {
                depth: 3,
                ..Default::default()
            },
        );
        assert!(move2_option.is_some());

        let move2 = move2_option.unwrap();
        assert_eq!(move2, ChessMove::from_xboard_algebraic_notation("e4d5"));
    }

    #[test]
    fn black_makes_better_move() {
        let mut game = Game::from_fen_notation("7k/8/4p3/3Q1N2/8/8/1P6/7K");
        game.move_piece(&ChessMove::from_xboard_algebraic_notation("b2b4"));

        let move1_option = alpha_beta(&game, Color::Black, &AlphaBetaParams::default());
        assert!(move1_option.is_some());

        let move1 = move1_option.unwrap();
        assert_eq!(move1, ChessMove::from_xboard_algebraic_notation("e6d5"));

        // try again with a alpha beta depth of 3
        let move2_option = alpha_beta(
            &game,
            Color::Black,
            &AlphaBetaParams {
                depth: 3,
                ..Default::default()
            },
        );
        assert!(move2_option.is_some());

        let move2 = move2_option.unwrap();
        assert_eq!(move2, ChessMove::from_xboard_algebraic_notation("e6d5"));
    }
}
