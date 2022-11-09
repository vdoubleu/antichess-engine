mod alpha_beta;
mod evaluate_game;
mod move_sort;
pub mod opening;
mod position_scores;
mod random;
pub mod store;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::alpha_beta::{alpha_beta, AlphaBetaParams};
use crate::engine::opening::OpeningBook;
use crate::engine::store::AlphaBetaStore;

pub fn generate_move(
    game: &Game,
    store: &mut AlphaBetaStore,
    color: Color,
    opening_book: Option<&OpeningBook>,
) -> Option<ChessMove> {
    // use opening book if available
    if game.turn_counter < 5 {
        if let Some(book) = opening_book {
            if let Some(m) = book.get_move(&game.get_fen_notation()) {
                return Some(m);
            }
        }
    }

    let mut params = AlphaBetaParams::default();

    let target_final_depth = params.depth;

    let mut best_move = None;

    let mut best_score = f64::NEG_INFINITY;

    store.start_turn();

    for curr_depth in 1..target_final_depth {
        if params.debug_print > 0 {
            eprintln!("Starting depth {}", curr_depth);
        }

        if let Some(start_time) = store.start_time {
            if start_time.elapsed() > params.max_time {
                break;
            }
        } else {
            panic!("start time not set");
        }

        params.depth = curr_depth;
        if let Some(res) = alpha_beta(game, color, &params, store) {
            best_move = Some(res.0);
            best_score = res.1;
        }

        if best_score == f64::INFINITY || best_score == f64::NEG_INFINITY {
            break;
        }
    }

    if params.debug_print > 0 {
        eprintln!(
            "search time: {}",
            store.start_time.unwrap().elapsed().as_millis()
        );
    }

    store.end_turn();

    best_move
}

#[cfg(test)]
mod gen_move_tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_gen_move_opening_book() {
        let mut store = AlphaBetaStore::new();
        let book = OpeningBook::new();
        let game = Game::new_starting_game();

        let curr_time = Instant::now();
        let m = generate_move(&game, &mut store, game.player_turn, Some(&book));

        let finish_time = curr_time.elapsed();

        assert!(m.is_some());

        // should use a move from the opening book, so it should be fast
        assert!(finish_time < Duration::from_millis(500));
    }
}
