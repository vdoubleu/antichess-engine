// use crate::chess_game::{ChessMove, Color, Game};
use crate::error::ChessError;

use anyhow::Result;

use pleco::{BitMove, Board};

use rand::prelude::SliceRandom;

pub fn random_move(board: &Board) -> Result<BitMove> {
    let all_valid_moves = board.generate_moves();
    let mut rng = rand::thread_rng();
    all_valid_moves
        .choose(&mut rng)
        .copied()
        .ok_or_else(|| ChessError::NoValidMoves.into())
}

#[cfg(test)]
mod random_move_tests {
    use super::*;
    // use crate::chess_game::Game;
    use anyhow::Result;

    #[test]
    fn test_random_move() -> Result<()> {
        let mut game = Board::start_pos();
        let move_1 = random_move(&game);
        assert!(move_1.is_ok());

        game.apply_move(move_1.unwrap());

        let move_2 = random_move(&game);
        assert!(move_2.is_ok());

        game.apply_move(move_2.unwrap());

        Ok(())
    }
}
