use crate::chess_game::valid_move_finder::bishop_move_finder::all_valid_moves_for_bishop;
use crate::chess_game::valid_move_finder::rook_move_finder::all_valid_moves_for_rook;
use crate::chess_game::{Game, Pos};

pub fn all_valid_moves_for_queen(game: &Game, pos: Pos) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let mut rook_moves = all_valid_moves_for_rook(game, pos);
    let mut bishop_moves = all_valid_moves_for_bishop(game, pos);

    valid_moves.append(&mut rook_moves);
    valid_moves.append(&mut bishop_moves);

    valid_moves
}

#[cfg(test)]
mod queen_tests {
    use super::*;
    use crate::chess_game::pos::PosExt;

    #[test]
    fn test_valid_moves() {
        let game = Game::from_fen_notation("8/8/8/8/3Q4/8/8/8");
        let queen_pos = Pos::from_alg_notation("d4");

        let valid_moves = all_valid_moves_for_queen(&game, queen_pos);

        assert_eq!(valid_moves.len(), 27);
    }
}
