use crate::chess_game::{CastleTypes, Color, Game, PieceType, Pos};

use crate::chess_game::pos::PosExt;

pub fn all_valid_moves_for_king(
    game: &Game,
    pos: Pos,
    color: Color,
    only_check_currently_attacking: bool,
) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let king_move_options: Vec<i8> = vec![1, -1, 9, -9, 10, -10, 11, -11];

    let (mut check_queen_castle, mut check_king_castle) = (false, false);

    for move_dist in king_move_options {
        let new_pos = (pos as i8 + move_dist) as Pos;

        // check underflow
        if !new_pos.on_board() {
            continue;
        }

        if game.board[new_pos].is_none() || game.has_piece_with_color(new_pos, color.opposite()) {
            valid_moves.push(new_pos);

            if move_dist == -1 {
                check_queen_castle = true;
            } else if move_dist == 1 {
                check_king_castle = true;
            }
        }
    }

    if only_check_currently_attacking {
        return valid_moves;
    }

    if pos.col() != 4 {
        return valid_moves;
    }

    // castling

    if check_queen_castle {
        let queen_castle_type = match color {
            Color::White => CastleTypes::WhiteQueen,
            Color::Black => CastleTypes::BlackQueen,
        };

        if game.castle_availability[queen_castle_type as usize] && valid_queen_castle(game, color) {
            valid_moves.push(pos - 2);
        }
    }

    if check_king_castle {
        let king_castle_type = match color {
            Color::White => CastleTypes::WhiteKing,
            Color::Black => CastleTypes::BlackKing,
        };

        if game.castle_availability[king_castle_type as usize] && valid_king_castle(game, color) {
            valid_moves.push(pos + 2);
        }
    }

    valid_moves
}

fn valid_king_castle(game: &Game, color: Color) -> bool {
    let king_rook_row = match color {
        Color::White => 7,
        Color::Black => 0,
    };
    let king_pos = Pos::new(king_rook_row, 4);
    let rook_pos = Pos::new(king_rook_row, 7);

    if let Some(maybe_rook) = game.get_piece(rook_pos) {
        if maybe_rook.piece_type != PieceType::Rook || maybe_rook.color != color {
            return false;
        }
    } else {
        return false;
    }

    if has_no_pieces_between(game, king_pos + 1, rook_pos - 1)
        && has_no_pieces_attacked_by_color(game, king_pos, king_pos + 2, color.opposite())
    {
        return true;
    }

    false
}

fn valid_queen_castle(game: &Game, color: Color) -> bool {
    let king_rook_row = match color {
        Color::White => 7,
        Color::Black => 0,
    };
    let king_pos = Pos::new(king_rook_row, 4);
    let rook_pos = Pos::new(king_rook_row, 0);

    if let Some(maybe_rook) = game.get_piece(rook_pos) {
        if maybe_rook.piece_type != PieceType::Rook || maybe_rook.color != color {
            return false;
        }
    } else {
        return false;
    }

    if has_no_pieces_between(game, rook_pos + 1, king_pos - 1)
        && has_no_pieces_attacked_by_color(game, king_pos - 2, king_pos, color.opposite())
    {
        return true;
    }

    false
}

fn has_no_pieces_attacked_by_color(
    game: &Game,
    start_pos: Pos,
    end_pos: Pos,
    color: Color,
) -> bool {
    for curr_pos in start_pos..=end_pos {
        if game.square_attacked_by_color(curr_pos, color) {
            return false;
        }
    }

    true
}

fn has_no_pieces_between(game: &Game, start_pos: Pos, end_pos: Pos) -> bool {
    for curr_pos in start_pos..=end_pos {
        if game.board[curr_pos].is_some() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test_king {
    use super::*;

    use crate::error::ChessError;

    #[test]
    fn test_king_move() -> Result<(), ChessError> {
        let mut game = Game::from_fen_notation("8/8/8/8/3k4/8/8/8")?;
        game.castle_availability = [false; 4];

        let king_pos = Pos::from_alg_notation("d4");

        let valid_moves = all_valid_moves_for_king(&game, king_pos, Color::Black, false);

        assert_eq!(valid_moves.len(), 8);

        Ok(())
    }

    #[test]
    fn test_castle() -> Result<(), ChessError> {
        let game = Game::from_fen_notation("r3k2r/8/8/8/8/8/8/8")?;
        let king_pos = Pos::from_alg_notation("e8");

        let valid_moves = all_valid_moves_for_king(&game, king_pos, Color::Black, false);

        assert_eq!(valid_moves.len(), 7);
        assert!(valid_moves.contains(&Pos::from_alg_notation("c8")));
        assert!(valid_moves.contains(&Pos::from_alg_notation("g8")));

        Ok(())
    }

    #[test]
    fn test_castle_block() -> Result<(), ChessError> {
        let game = Game::from_fen_notation("rn2k1nr/8/8/8/8/8/8/8")?;
        let king_pos = Pos::from_alg_notation("e8");

        let valid_moves = all_valid_moves_for_king(&game, king_pos, Color::Black, false);

        assert_eq!(valid_moves.len(), 5);
        assert!(!valid_moves.contains(&Pos::from_alg_notation("c8")));
        assert!(!valid_moves.contains(&Pos::from_alg_notation("g8")));

        Ok(())
    }

    #[test]
    fn test_castle_king_check() -> Result<(), ChessError> {
        let game = Game::from_fen_notation("r3k2r/8/2B5/8/8/8/8/8")?;
        let king_pos = Pos::from_alg_notation("e8");

        let valid_moves = all_valid_moves_for_king(&game, king_pos, Color::Black, false);

        assert_eq!(valid_moves.len(), 5);
        assert!(!valid_moves.contains(&Pos::from_alg_notation("c8")));
        assert!(!valid_moves.contains(&Pos::from_alg_notation("g8")));

        Ok(())
    }

    #[test]
    fn test_castle_king_through_check() -> Result<(), ChessError> {
        let game = Game::from_fen_notation("r3k2r/8/8/8/2B2B2/8/8/8")?;
        let king_pos = Pos::from_alg_notation("e8");

        let valid_moves = all_valid_moves_for_king(&game, king_pos, Color::Black, false);

        assert_eq!(valid_moves.len(), 6);
        assert!(valid_moves.contains(&Pos::from_alg_notation("c8")));
        assert!(!valid_moves.contains(&Pos::from_alg_notation("g8")));

        Ok(())
    }
}
