use crate::engine::evaluate_game::evaluate;
use crate::engine::move_sort::sort_moves;
use crate::engine::store::TranspositionTableFlag;
use crate::engine::Engine;

use crate::error::ChessError;

use anyhow::Result;
use pleco::{BitMove, Board, Player};

/// Implements the min max algorithm (without alpha beta pruning for now) to decide the best move
/// to play. White is maximizing, black is minimizing.
pub fn alpha_beta(board: &Board, engine: &mut Engine) -> Result<(BitMove, f64)> {
    let reasonable_depth = engine.store.curr_depth;
    let max_depth = engine.params.max_depth;

    let mut best_move: Option<BitMove> = None;

    let mut alpha = f64::NEG_INFINITY;
    let beta = f64::INFINITY;

    let mut best_score = f64::NEG_INFINITY;

    let mut all_valid_moves = engine.generate_valid_moves(board);
    let valid_moves_len = all_valid_moves.len();

    if all_valid_moves.is_empty() {
        return Err(ChessError::NoValidMoves.into());
    }

    // move ordering
    all_valid_moves = sort_moves(board, &engine.store, &all_valid_moves);

    let mut new_board = board.clone();

    let mut ind = 1;
    for chess_move in all_valid_moves {
        if engine.params.debug_print > 1 {
            eprintln!(
                "Trying move {} of {} - {}",
                ind, valid_moves_len, chess_move
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
            return Err(ChessError::NoStartTime.into());
        }

        new_board.apply_move(chess_move);

        let eval_res = alpha_beta_impl(
            &mut new_board,
            -beta,
            -alpha,
            reasonable_depth, //+ (ind % 2),
            max_depth,
            true,
            engine,
        );

        new_board.undo_move();

        if eval_res.is_err() {
            if engine.params.handle_errors {
                // if we get an error, we just continue with the best move we have so far
                continue;
            } else {
                return Err(eval_res.err().unwrap());
            }
        }

        let eval = -eval_res.unwrap();

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
        .store_transposition(board, reasonable_depth, best_score, best_move, node_type);

    engine.store.probe_fill_pv(&mut new_board)?;

    if let Some(best_move) = best_move {
        Ok((best_move, best_score))
    } else {
        Err(ChessError::NoMoveGenerated.into())
    }
}

#[allow(clippy::too_many_arguments)]
fn alpha_beta_impl(
    board: &mut Board,
    alpha: f64,
    beta: f64,
    curr_depth: i32,
    max_depth: i32,
    do_null: bool,
    engine: &mut Engine,
) -> Result<f64> {
    if curr_depth <= 0 || max_depth <= 0 || board.checkmate() || board.stalemate() {
        let pov = if board.turn() == Player::White {
            1.0
        } else {
            -1.0
        };

        return Ok(pov * evaluate(board));
    }

    let mut curr_alpha = alpha;
    let mut curr_beta = beta;

    if let Some(transpo) = engine.store.get_transposition(board) {
        if board.fen() == transpo.fen && transpo.depth >= curr_depth {
            match transpo.flag {
                TranspositionTableFlag::Exact => {
                    return Ok(transpo.score);
                }
                TranspositionTableFlag::Upper => {
                    curr_alpha = curr_alpha.max(transpo.score);
                }
                TranspositionTableFlag::Lower => {
                    curr_beta = curr_beta.min(transpo.score);
                }
            }

            if curr_alpha >= curr_beta {
                return Ok(transpo.score);
            }
        }
    }

    // do null move pruning if we are at a reasonable depth
    // and the game is not over
    if do_null
        && board.ply() > 0
        && curr_depth >= 4
        && engine.params.null_move_reduction > 0
        && !board.in_check()
    {
        unsafe {
            board.apply_null_move();
        }

        let eval_res = alpha_beta_impl(
            board,
            -curr_beta,
            -curr_beta + 1.0,
            curr_depth - engine.params.null_move_reduction,
            max_depth - 1,
            false,
            engine,
        );

        unsafe {
            // board.undo_move();
            board.undo_null_move();
        }

        // if we get an error, we just continue on, we don't do null move pruning
        if let Ok(eval_out) = eval_res {
            let eval = -eval_out;

            if eval >= curr_beta && eval.abs() < f64::INFINITY {
                return Ok(curr_beta);
            }
        }
    }

    let mut all_valid_moves = engine.generate_valid_moves(board);
    let valid_moves_len = all_valid_moves.len();

    // move ordering
    all_valid_moves = sort_moves(board, &engine.store, &all_valid_moves);

    let new_curr_depth = if valid_moves_len <= 3 {
        curr_depth
    } else {
        curr_depth - 1
    };

    let mut score = f64::NEG_INFINITY;

    let mut best_move = None;

    for move_option in all_valid_moves {
        if engine.params.debug_print > 1 {
            eprintln!("move: {} {} {:?}", board.turn(), move_option, move_option);
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
            return Err(ChessError::NoStartTime.into());
        }

        board.apply_move(move_option);

        let eval_res = alpha_beta_impl(
            board,
            -curr_beta,
            -curr_alpha,
            new_curr_depth,
            max_depth - 1,
            true,
            engine,
        );

        board.undo_move();

        if eval_res.is_err() {
            if engine.params.handle_errors {
                // if we get an error, we just continue with the best move we have so far
                continue;
            } else {
                return Err(eval_res.err().unwrap());
            }
        }

        let eval = -eval_res.unwrap();

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
        .store_transposition(board, curr_depth, score, best_move, node_type);

    Ok(score)
}

#[cfg(test)]
mod alpha_beta_tests {
    use super::*;
    // use crate::chess_game::{Color, Game};
    use pleco::{Board, SQ};

    use anyhow::bail;

    use std::time::Duration;

    #[test]
    fn test_alpha_beta() -> Result<()> {
        let mut game = Board::start_pos();
        let mut engine = Engine::new();
        engine.params.depth = 5;
        engine.params.max_time = Duration::from_secs(8);

        engine.store.curr_depth = engine.params.depth;

        let move1 = alpha_beta(&game, &mut engine);
        assert!(move1.is_ok());

        game.apply_move(move1.unwrap().0);

        Ok(())
    }

    #[test]
    fn white_makes_better_move() -> Result<()> {
        let game = match Board::from_fen("7k/8/8/3q1n2/4P3/8/8/7K w - - 0 1") {
            Ok(g) => g,
            Err(e) => bail!("can't build fen: {:?}", e),
        };

        let mut engine = Engine::new();
        engine.store.curr_depth = engine.params.depth;

        let move1_option = alpha_beta(&game, &mut engine);
        assert!(move1_option.is_ok());

        let move1 = move1_option.unwrap().0;
        assert!(move1.get_dest() == SQ(35));
        assert!(move1.get_src() == SQ(28));
        assert!(move1.is_capture());

        // try again with a alpha beta depth of 3
        let mut engine2 = Engine::new();
        engine2.params.depth = 3;

        let move2_option = alpha_beta(&game, &mut engine2);
        assert!(move2_option.is_ok());

        let move2 = move2_option.unwrap().0;
        assert!(move2.get_dest() == SQ(35));
        assert!(move2.get_src() == SQ(28));
        assert!(move2.is_capture());

        Ok(())
    }

    #[test]
    fn black_makes_better_move() -> Result<()> {
        let game = match Board::from_fen("7k/8/8/4p3/3Q1N2/8/8/7K b - - 0 1") {
            Ok(g) => g,
            Err(e) => bail!("can't build fen: {:?}", e),
        };
        let mut engine = Engine::new();
        engine.store.curr_depth = engine.params.depth;

        let move1_option = alpha_beta(&game, &mut engine);
        assert!(move1_option.is_ok());

        let move1 = move1_option.unwrap().0;
        assert!(move1.get_src() == SQ(36));
        assert!(move1.get_dest() == SQ(27));
        assert!(move1.is_capture());

        // try again with a alpha beta depth of 3
        let mut engine2 = Engine::new();
        engine2.params.depth = 3;

        let move2_option = alpha_beta(&game, &mut engine2);
        assert!(move2_option.is_ok());

        let move2 = move2_option.unwrap().0;
        assert!(move2.get_src() == SQ(36));
        assert!(move2.get_dest() == SQ(27));
        assert!(move2.is_capture());

        Ok(())
    }
}
