use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::evaluate_game::evaluate;
use crate::engine::move_sort::sort_moves;
use crate::engine::store::TranspositionTableFlag;
use crate::engine::Engine;

/// Implements the min max algorithm (without alpha beta pruning for now) to decide the best move
/// to play. White is maximizing, black is minimizing.
pub fn alpha_beta(game: &Game, color: Color, engine: &mut Engine) -> Option<(ChessMove, f64)> {
    let reasonable_depth = engine.store.curr_depth;
    let max_depth = engine.params.max_depth;

    let mut best_move: Option<ChessMove> = None;

    let mut alpha = f64::NEG_INFINITY;
    let beta = f64::INFINITY;

    let mut best_score = f64::NEG_INFINITY;

    let mut all_valid_moves = game.all_valid_moves_for_color(color);
    let valid_moves_len = all_valid_moves.len();

    if all_valid_moves.is_empty() {
        return None;
    }

    let mut new_game = game.clone();

    // move ordering
    all_valid_moves = sort_moves(&new_game, &engine.store, &all_valid_moves);

    let mut ind = 1;
    for chess_move in all_valid_moves {
        if engine.params.debug_print > 1 {
            eprintln!("Trying move {} of {}", ind, valid_moves_len);
        }

        if let Some(start_time) = engine.store.start_time {
            if start_time.elapsed() > engine.params.max_time {
                if engine.params.debug_print > 0 {
                    eprintln!(
                        "Search time limit reached: {}",
                        engine.params.max_time.as_secs()
                    );
                }
                break;
            }
        } else {
            panic!("start time not set");
        }

        new_game.move_piece(&chess_move);

        let eval = -alpha_beta_impl(
            &mut new_game,
            -beta,
            -alpha,
            reasonable_depth, //+ (ind % 2),
            max_depth,
            true,
            engine,
        );

        new_game.unmove_move();

        if eval == best_score && best_move.is_none() {
            best_move = Some(chess_move)
        } else if eval > best_score {
            best_score = eval;
            best_move = Some(chess_move);
        }

        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            break;
        }

        ind += 1;
    }

    let node_type = if best_score <= alpha {
        TranspositionTableFlag::Upper
    } else if best_score >= beta {
        TranspositionTableFlag::Lower
    } else {
        TranspositionTableFlag::Exact
    };

    engine
        .store
        .store_transposition(game, reasonable_depth, best_score, best_move, node_type);

    engine.store.probe_fill_pv(&mut new_game);

    best_move.map(|m| (m, best_score))
}

#[allow(clippy::too_many_arguments)]
fn alpha_beta_impl(
    game: &mut Game,
    alpha: f64,
    beta: f64,
    curr_depth: i32,
    max_depth: i32,
    do_null: bool,
    engine: &mut Engine,
) -> f64 {
    if curr_depth <= 0 || max_depth <= 0 || game.winner.is_some() {
        let pov = if game.player_turn == Color::White {
            1.0
        } else {
            -1.0
        };

        return pov * evaluate(game);
    }

    let mut curr_alpha = alpha;
    let mut curr_beta = beta;

    if let Some(transpo) = engine.store.get_transposition(game) {
        if game.get_fen_notation() == transpo.fen && transpo.depth >= curr_depth {
            match transpo.flag {
                TranspositionTableFlag::Exact => {
                    return transpo.score;
                }
                TranspositionTableFlag::Upper => {
                    curr_alpha = curr_alpha.max(transpo.score);
                }
                TranspositionTableFlag::Lower => {
                    curr_beta = curr_beta.min(transpo.score);
                }
            }

            if curr_alpha >= curr_beta {
                return transpo.score;
            }
        }
    }

    // do null move pruning if we are at a reasonable depth
    // and the game is not over
    if do_null && game.turn_counter > 0 && curr_depth >= 4 && engine.params.null_move_reduction > 0
    {
        game.move_piece(&ChessMove::new_null_move());

        let eval = -alpha_beta_impl(
            game,
            -curr_beta,
            -curr_beta + 1.0,
            curr_depth - engine.params.null_move_reduction,
            max_depth - 1,
            false,
            engine,
        );

        game.unmove_move();

        if eval >= curr_beta && eval.abs() < f64::INFINITY {
            return curr_beta;
        }
    }

    let mut all_valid_moves = game.all_valid_moves_for_color(game.player_turn);
    let valid_moves_len = all_valid_moves.len();

    // move ordering
    all_valid_moves = sort_moves(game, &engine.store, &all_valid_moves);

    let new_curr_depth = if valid_moves_len <= 3 {
        curr_depth
    } else {
        curr_depth - 1
    };

    let mut score = f64::NEG_INFINITY;

    let mut best_move = None;

    for move_option in all_valid_moves {
        if engine.params.debug_print > 1 {
            eprintln!(
                "move: {} {} {:?}",
                game.player_turn, move_option, move_option
            );
        }

        if let Some(start_time) = engine.store.start_time {
            if start_time.elapsed() > engine.params.max_time {
                if engine.params.debug_print > 0 {
                    eprintln!(
                        "Search time limit reached: {}",
                        engine.params.max_time.as_secs()
                    );
                }
                break;
            }
        } else {
            panic!("start time not set");
        }

        game.move_piece(&move_option);

        let eval = -alpha_beta_impl(
            game,
            -curr_beta,
            -curr_alpha,
            new_curr_depth,
            max_depth - 1,
            true,
            engine,
        );

        game.unmove_move();

        if eval > score {
            score = eval;
            best_move = Some(move_option);
        }
        if score > curr_alpha {
            curr_alpha = score;
        }
        if curr_alpha >= curr_beta {
            break;
        }
    }

    let node_type = if score <= alpha {
        TranspositionTableFlag::Upper
    } else if score >= beta {
        TranspositionTableFlag::Lower
    } else {
        TranspositionTableFlag::Exact
    };

    engine
        .store
        .store_transposition(game, curr_depth, score, best_move, node_type);

    score
}

#[cfg(test)]
mod alpha_beta_tests {
    use super::*;
    use crate::chess_game::{Color, Game};

    #[test]
    fn test_alpha_ceta() {
        let mut game = Game::new_starting_game();
        let mut engine = Engine::new();
        engine.store.curr_depth = engine.params.depth;

        let move1 = alpha_beta(&game, Color::White, &mut engine);
        assert!(move1.is_some());

        game.move_piece(&move1.unwrap().0);
    }

    #[test]
    fn white_makes_better_move() {
        let game = Game::from_fen_notation("7k/8/8/3q1n2/4P3/8/8/7K");
        let mut engine = Engine::new();
        engine.store.curr_depth = engine.params.depth;

        let move1_option = alpha_beta(&game, Color::White, &mut engine);
        assert!(move1_option.is_some());

        let move1 = move1_option.unwrap().0;
        assert_eq!(move1, ChessMove::from_xboard_algebraic_notation("e4d5"));

        // try again with a alpha beta depth of 3
        let mut engine2 = Engine::new();
        engine2.params.depth = 3;

        let move2_option = alpha_beta(&game, Color::White, &mut engine2);
        assert!(move2_option.is_some());

        let move2 = move2_option.unwrap().0;
        assert_eq!(move2, ChessMove::from_xboard_algebraic_notation("e4d5"));
    }

    #[test]
    fn black_makes_better_move() {
        let mut game = Game::from_fen_notation("7k/8/4p3/3Q1N2/8/8/1P6/7K");
        let mut engine = Engine::new();
        engine.store.curr_depth = engine.params.depth;

        game.move_piece(&ChessMove::from_xboard_algebraic_notation("b2b4"));

        let move1_option = alpha_beta(&game, Color::Black, &mut engine);
        assert!(move1_option.is_some());

        let move1 = move1_option.unwrap().0;
        assert_eq!(move1, ChessMove::from_xboard_algebraic_notation("e6d5"));

        // try again with a alpha beta depth of 3
        let mut engine2 = Engine::new();
        let move2_option = alpha_beta(&game, Color::Black, &mut engine2);
        assert!(move2_option.is_some());

        let move2 = move2_option.unwrap().0;
        assert_eq!(move2, ChessMove::from_xboard_algebraic_notation("e6d5"));
    }
}
