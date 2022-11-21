mod alpha_beta;
mod evaluate_game;
mod move_sort;
pub mod opening;
mod position_scores;
mod random;
pub mod store;

use crate::engine::alpha_beta::alpha_beta;
use crate::engine::opening::OpeningBook;
use crate::engine::random::random_move;
use crate::engine::store::AlphaBetaStore;
use crate::error::ChessError;

use anyhow::Result;
use pleco::{BitMove, Board};

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
    /// handle the errors instead of panicing
    pub handle_errors: bool,

    /// total time allowed to be spent searching
    pub total_time: Duration,
}

impl Default for AlphaBetaParams {
    fn default() -> Self {
        AlphaBetaParams {
            depth: 7,
            max_depth: 28,
            null_move_reduction: 2,
            debug_print: 1,
            max_time: Duration::from_secs(25),
            handle_errors: true,
            total_time: Duration::from_secs(180),
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

    fn dynamic_depth_calculator(&self, depth_estimate: i32) -> i32 {
        let time_left_dur =
            self.params.total_time - Duration::from_millis(self.store.total_search_time_ms as u64);
        let time_left_secs = time_left_dur.as_secs_f64();

        if time_left_secs > 45.0 {
            depth_estimate
        } else if time_left_secs > 20.0 {
            depth_estimate - 1
        } else if time_left_secs > 10.0 {
            depth_estimate - 2
        } else {
            depth_estimate - 3
        }
    }

    /// Generates a move using the alpha beta algorithm
    /// This uses several optimizations to make it faster.
    ///
    /// This includes optimizations such as:
    /// - Null move pruning
    /// - Transposition table
    /// - Move ordering
    /// - Principal variation search
    /// - Iterative deepening (inside the main search function inside mod.rs)
    /// - Time management (will dynamically adjust depth based on time left)
    pub fn generate_move(&mut self, board: &Board) -> Result<BitMove> {
        // use opening book if available
        if board.ply() < 5 && self.opening_book.is_some() {
            let book: &OpeningBook = self.opening_book.as_ref().unwrap();
            if let Some(m) = book.get_move(&board.fen()) {
                return Ok(m);
            }
        }

        let target_final_depth = self.dynamic_depth_calculator(self.params.depth);

        if self.params.debug_print > 0 {
            eprintln!("searching to depth {}", target_final_depth);
        }

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
            } else if self.params.handle_errors {
                return Err(ChessError::NoStartTime.into());
            } else {
                panic!("No start time");
            }

            self.store.curr_depth = curr_depth;
            if let Ok(res) = alpha_beta(board, self) {
                best_move = Some(res.0);
                best_score = res.1;
            } else if !self.params.handle_errors {
                panic!("Alpha beta error");
            } // else we just ignore the error and keep going

            if best_score == f64::INFINITY || best_score == f64::NEG_INFINITY {
                break;
            }
        }

        let search_time = self.store.start_time.unwrap().elapsed().as_millis();
        self.store.total_search_time_ms += search_time;

        if self.params.debug_print > 0 {
            eprintln!(
                "search time: {}, total_time: {}",
                search_time, self.store.total_search_time_ms
            );
        }

        self.store.end_turn();

        if let Some(best_move_res) = best_move {
            Ok(best_move_res)
        } else {
            Err(ChessError::NoMoveGenerated.into())
        }
    }

    pub fn generate_rand_move(&self, board: &Board) -> Result<BitMove> {
        random_move(board)
    }

    pub fn generate_valid_moves(&self, board: &Board) -> Vec<BitMove> {
        let moves = board.generate_moves();

        // get two lists of moves, one for captures and one for non-captures
        let mut captures = Vec::new();

        for m in moves.iter() {
            if board.is_capture(*m) {
                captures.push(*m);
            }
        }

        if captures.is_empty() {
            moves.into_iter().collect()
        } else {
            captures
        }
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
        let game = Board::start_pos();
        let mut engine = Engine::new();
        engine.params.depth = 10;
        engine.opening_book = Some(OpeningBook::new());

        let curr_time = Instant::now();
        let m = engine.generate_move(&game);

        let finish_time = curr_time.elapsed();

        assert!(m.is_ok());

        assert_eq!(
            m.unwrap(),
            engine.opening_book.unwrap().get_move(&game.fen()).unwrap()
        );

        // should use a move from the opening book, so it should be fast
        assert!(finish_time < Duration::from_millis(500));
    }
}
