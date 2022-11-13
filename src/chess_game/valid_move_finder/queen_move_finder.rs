use crate::chess_game::valid_move_finder::bishop_move_finder::all_valid_moves_for_bishop;
use crate::chess_game::valid_move_finder::rook_move_finder::all_valid_moves_for_rook;
use crate::chess_game::{Color, Game, Pos};

pub fn all_valid_moves_for_queen(game: &Game, pos: Pos, color: Color) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let mut rook_moves = all_valid_moves_for_rook(game, pos, color);
    let mut bishop_moves = all_valid_moves_for_bishop(game, pos, color);

    valid_moves.append(&mut rook_moves);
    valid_moves.append(&mut bishop_moves);

    valid_moves
}

#[cfg(test)]
mod queen_tests {
    use super::*;
    use crate::chess_game::pos::PosExt;
    use crate::error::ChessError;

    #[test]
    fn test_valid_moves() -> Result<(), ChessError> {
        let game = Game::from_fen_notation("8/8/8/8/3Q4/8/8/8")?;
        let queen_pos = Pos::from_alg_notation("d4");

        let valid_moves = all_valid_moves_for_queen(&game, queen_pos, Color::White);

        assert_eq!(valid_moves.len(), 27);

        Ok(())
    }
}
