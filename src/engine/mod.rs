mod alpha_beta;
mod evaluate_game;
mod move_sort;
pub mod opening;
mod position_scores;
mod random;
pub mod store;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::alpha_beta::alpha_beta;
use crate::engine::opening::OpeningBook;
use crate::engine::store::AlphaBetaStore;
use std::time::Duration;

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
    pub debug_print: i8,
    /// the maximum amount of time to search for
    pub max_time: Duration,
}

impl Default for AlphaBetaParams {
    fn default() -> Self {
        AlphaBetaParams {
            depth: 6,
            max_depth: 16,
            null_move_reduction: 2,
            debug_print: 1,
            max_time: Duration::from_secs(15),
        }
    }
}

pub struct Engine {
    pub opening_book: Option<OpeningBook>,
    pub store: AlphaBetaStore,
    pub params: AlphaBetaParams,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            opening_book: None,
            store: AlphaBetaStore::new(),
            params: AlphaBetaParams::default(),
        }
    }

    pub fn generate_move(&mut self, game: &Game, color: Color) -> Option<ChessMove> {
        // use opening book if available
        if game.turn_counter < 5 && self.opening_book.is_some() {
            let book: &OpeningBook = self.opening_book.as_ref().unwrap();
            if let Some(m) = book.get_move(&game.get_fen_notation()) {
                return Some(m);
            }
        }

        let target_final_depth = self.params.depth;

        let mut best_move = None;

        let mut best_score = f64::NEG_INFINITY;

        self.store.start_turn();

        for curr_depth in 1..target_final_depth {
            if self.params.debug_print > 0 {
                eprintln!("Starting depth {}", curr_depth);
            }

            if let Some(start_time) = self.store.start_time {
                if start_time.elapsed() > self.params.max_time {
                    if self.params.debug_print > 0 {
                        eprintln!(
                            "Search time limit reached: {}",
                            self.params.max_time.as_secs()
                        );
                    }
                    break;
                }
            } else {
                panic!("start time not set");
            }

            self.store.curr_depth = curr_depth;
            if let Some(res) = alpha_beta(game, color, self) {
                best_move = Some(res.0);
                best_score = res.1;
            }

            if best_score == f64::INFINITY || best_score == f64::NEG_INFINITY {
                break;
            }
        }

        if self.params.debug_print > 0 {
            eprintln!(
                "search time: {}",
                self.store.start_time.unwrap().elapsed().as_millis()
            );
        }

        self.store.end_turn();

        best_move
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod gen_move_tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_gen_move_opening_book() {
        let game = Game::new_starting_game();
        let mut engine = Engine::new();
        engine.params.depth = 10;
        engine.opening_book = Some(OpeningBook::new());

        let curr_time = Instant::now();
        let m = engine.generate_move(&game, game.player_turn);

        let finish_time = curr_time.elapsed();

        assert!(m.is_some());

        assert_eq!(
            m.unwrap(),
            engine
                .opening_book
                .unwrap()
                .get_move(&game.get_fen_notation())
                .unwrap()
        );

        // should use a move from the opening book, so it should be fast
        assert!(finish_time < Duration::from_millis(500));
    }
}
