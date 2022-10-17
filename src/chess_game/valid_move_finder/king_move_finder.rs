use crate::chess_game::valid_move_finder::move_within_board_bounds;
use crate::chess_game::{Color, Game, PieceType, Pos};

pub fn all_valid_moves_for_king(
    game: &Game,
    pos: &Pos,
    only_check_currently_attacking: bool,
) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let row = pos.row;
    let col = pos.col;

    let piece = match game.piece_at(row, col) {
        Some(p) => p,
        None => return valid_moves,
    };

    if piece.piece_type != PieceType::King {
        return valid_moves;
    }

    let color = piece.color;

    let king_move_options: Vec<(i16, i16)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    for (row_offset, col_offset) in king_move_options {
        let new_row = row as i16 + row_offset;
        let new_col = col as i16 + col_offset;

        // check underflow
        if !move_within_board_bounds(new_row, new_col) {
            continue;
        }

        let m = Pos::new(new_row as usize, new_col as usize);
        if game.is_empty_square(&m) {
            valid_moves.push(m);
        } else if game.has_piece_with_color(&m, color) {
            continue;
        } else {
            valid_moves.push(m);
        }
    }

    if only_check_currently_attacking {
        return valid_moves;
    }

    // castling
    // check if king has moved
    if piece.has_moved() {
        return valid_moves;
    }

    // check if rooks have moved
    let rook_pos = match color {
        Color::White => vec![Pos::new(7, 0), Pos::new(7, 7)],
        Color::Black => vec![Pos::new(0, 0), Pos::new(0, 7)],
    };

    for pos in rook_pos {
        let rook_piece = match game.piece_at_pos(&pos) {
            Some(p) => p,
            None => continue,
        };

        if rook_piece.has_moved() {
            continue;
        }

        // check if there are any pieces between the king and the rook
        let (_king_row, king_col) = match color {
            Color::White => (7, 4),
            Color::Black => (0, 4),
        };

        let (rook_row, rook_col) = (pos.row, pos.col);

        let (start, end, king_check_start, king_check_end, king_dest_col) = if rook_col < king_col {
            (rook_col + 1, king_col, rook_col + 2, king_col, rook_col + 2) //queen side castle
        } else {
            (king_col + 1, rook_col, king_col, rook_col - 1, rook_col - 1) // king side castle
        };

        let mut has_pieces_between = false;
        for i in start..end {
            if !game.is_empty_square(&Pos::new(rook_row, i)) {
                has_pieces_between = true;
                break;
            }
        }

        if has_pieces_between {
            continue;
        }

        // check if the squares the king will move through are attacked
        // or if king is in check
        let mut squares_attacked = false;

        for i in king_check_start..=king_check_end {
            if game.square_attacked_by_color(&Pos::new(rook_row, i), color.opposite()) {
                squares_attacked = true;
                break;
            }
        }

        if squares_attacked {
            continue;
        }

        // if we get here, then the king can castle
        valid_moves.push(Pos::new(rook_row, king_dest_col));
    }

    valid_moves
}

#[cfg(test)]
mod test_king {
    use super::*;

    #[test]
    fn test_king_move() {
        let game = Game::from_fen_notation("8/8/8/8/3k4/8/8/8".to_string());
        let king_pos = Pos::from_algebraic_notation("d4".to_string());

        let valid_moves = all_valid_moves_for_king(&game, &king_pos, false);

        assert_eq!(valid_moves.len(), 8);
    }

    #[test]
    fn test_castle() {
        let game = Game::from_fen_notation("r3k2r/8/8/8/8/8/8/8".to_string());
        let king_pos = Pos::from_algebraic_notation("e8".to_string());

        let valid_moves = all_valid_moves_for_king(&game, &king_pos, false);

        assert_eq!(valid_moves.len(), 7);
        assert!(valid_moves.contains(&Pos::from_algebraic_notation("c8".to_string())));
        assert!(valid_moves.contains(&Pos::from_algebraic_notation("g8".to_string())));
    }

    #[test]
    fn test_castle_block() {
        let game = Game::from_fen_notation("rn2k1nr/8/8/8/8/8/8/8".to_string());
        let king_pos = Pos::from_algebraic_notation("e8".to_string());

        let valid_moves = all_valid_moves_for_king(&game, &king_pos, false);

        assert_eq!(valid_moves.len(), 5);
        assert!(!valid_moves.contains(&Pos::from_algebraic_notation("c8".to_string())));
        assert!(!valid_moves.contains(&Pos::from_algebraic_notation("g8".to_string())));
    }

    #[test]
    fn test_castle_king_check() {
        let game = Game::from_fen_notation("r3k2r/8/2B5/8/8/8/8/8".to_string());
        let king_pos = Pos::from_algebraic_notation("e8".to_string());

        let valid_moves = all_valid_moves_for_king(&game, &king_pos, false);

        assert_eq!(valid_moves.len(), 5);
        assert!(!valid_moves.contains(&Pos::from_algebraic_notation("c8".to_string())));
        assert!(!valid_moves.contains(&Pos::from_algebraic_notation("g8".to_string())));
    }

    #[test]
    fn test_castle_king_through_check() {
        let game = Game::from_fen_notation("r3k2r/8/8/8/2B2B2/8/8/8".to_string());
        let king_pos = Pos::from_algebraic_notation("e8".to_string());

        let valid_moves = all_valid_moves_for_king(&game, &king_pos, false);

        assert_eq!(valid_moves.len(), 6);
        assert!(valid_moves.contains(&Pos::from_algebraic_notation("c8".to_string())));
        assert!(!valid_moves.contains(&Pos::from_algebraic_notation("g8".to_string())));
    }
}
