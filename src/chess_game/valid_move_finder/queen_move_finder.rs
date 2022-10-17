use crate::chess_game::valid_move_finder::bishop_move_finder::all_valid_moves_for_bishop;
use crate::chess_game::valid_move_finder::rook_move_finder::all_valid_moves_for_rook;
use crate::chess_game::{Game, PieceType, Pos};

pub fn all_valid_moves_for_queen(game: &Game, pos: &Pos) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let row = pos.row;
    let col = pos.col;

    let piece = match game.piece_at(row, col) {
        Some(p) => p,
        None => return valid_moves,
    };

    if piece.piece_type != PieceType::Queen {
        return valid_moves;
    }

    let mut rook_moves = all_valid_moves_for_rook(game, pos, true);
    let mut bishop_moves = all_valid_moves_for_bishop(game, pos, true);

    valid_moves.append(&mut rook_moves);
    valid_moves.append(&mut bishop_moves);

    valid_moves
}

#[cfg(test)]
mod queen_tests {
    use super::*;

    #[test]
    fn test_valid_moves() {
        let game = Game::from_fen_notation("8/8/8/8/3Q4/8/8/8".to_string());
        let queen_pos = Pos::from_algebraic_notation("d4".to_string());

        let valid_moves = all_valid_moves_for_queen(&game, &queen_pos);

        assert_eq!(valid_moves.len(), 27);
    }
}
