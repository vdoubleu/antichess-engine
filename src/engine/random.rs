use crate::error::ChessError;

use anyhow::Result;

use pleco::{BitMove, Board};

use crate::engine::Engine;

use rand::prelude::SliceRandom;

pub fn random_move(board: &Board, engine: &Engine) -> Result<BitMove> {
    let all_valid_moves = engine.generate_valid_moves(board);
    let mut rng = rand::thread_rng();
    all_valid_moves
        .choose(&mut rng)
        .copied()
        .ok_or_else(|| ChessError::NoValidMoves.into())
}

#[cfg(test)]
mod random_move_tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_random_move() -> Result<()> {
        let engine = Engine::new();
        let mut game = Board::start_pos();
        let move_1 = random_move(&game, &engine);
        assert!(move_1.is_ok());

        game.apply_move(move_1.unwrap());

        let move_2 = random_move(&game, &engine);
        assert!(move_2.is_ok());

        game.apply_move(move_2.unwrap());

        Ok(())
    }
}
