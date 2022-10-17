use crate::chess_game::valid_move_finder::move_within_board_bounds;
use crate::chess_game::{Color, Game, PieceType, Pos};

pub fn all_valid_moves_for_pawn(game: &Game, pos: &Pos) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let row = pos.row;
    let col = pos.col;

    let piece = match game.piece_at(row, col) {
        Some(p) => p,
        None => return valid_moves,
    };

    if piece.piece_type != PieceType::Pawn {
        return valid_moves;
    }

    let color = piece.color;
    let has_moved = piece.has_moved();

    let pawn_moves: Vec<(i16, i16)> = match color {
        Color::White => vec![(-1, 0), (-2, 0), (-1, 1), (-1, -1)],
        Color::Black => vec![(1, 0), (2, 0), (1, -1), (1, 1)],
    };

    for (row_offset, col_offset) in pawn_moves {
        let new_row = (row as i16) + row_offset;
        let new_col = (col as i16) + col_offset;

        if !move_within_board_bounds(new_row, new_col) {
            continue;
        }

        let new_pos = Pos::new(new_row as usize, new_col as usize);

        // if moving diagonally, check if there is a piece to capture
        // either capture directly, or en passant
        if col_offset != 0 && is_en_passant(game, pos, &new_pos) {
            valid_moves.push(new_pos);
            continue;
        }

        if col_offset != 0 && game.is_empty_square(&new_pos) {
            continue;
        }

        // if moving forward, check if there is a piece in the way
        if col_offset == 0 && row_offset.abs() >= 1 && !game.is_empty_square(&new_pos) {
            continue;
        }

        if col_offset == 0 && row_offset.abs() == 2 {
            if has_moved {
                continue;
            }

            let row_offset_2 = match color {
                Color::White => -1,
                Color::Black => 1,
            };

            let in_front = Pos::new((row as i16 + row_offset_2) as usize, col);
            if !game.is_empty_square(&in_front) {
                continue;
            }
        }

        valid_moves.push(new_pos);
    }

    valid_moves
}

fn is_en_passant(game: &Game, pawn_curr_pos: &Pos, move_pos: &Pos) -> bool {
    let target_pos = Pos::new(pawn_curr_pos.row, move_pos.col);
    // there is a piece that just moved beside us
    let target_piece = match game.piece_at(target_pos.row, target_pos.col) {
        Some(p) => p,
        None => return false,
    };

    // the piece is a pawn
    if target_piece.piece_type != PieceType::Pawn {
        return false;
    }

    // the piece is on the other side
    if target_piece.color == game.player_turn {
        return false;
    }

    // the piece has just moved
    if target_piece.last_moved != game.turn_counter - 1 {
        return false;
    }

    // the piece just moved 2 spaces
    if target_piece.last_moved_from.is_none()
        || ((target_piece.last_moved_from.unwrap().row as i16) - (pawn_curr_pos.row as i16)).abs()
            != 2
    {
        return false;
    }

    true
}

#[cfg(test)]
mod pawn_tests {
    use super::*;

    #[test]
    fn test_valid_pawn_moves_black() {
        let game = Game::from_fen_notation("8/5p2/4P1P1/8/8/8/8/8".to_string());
        let pos = Pos::new(1, 5);

        let valid_moves = all_valid_moves_for_pawn(&game, &pos);
        let expected_moves = vec![
            Pos::new(2, 5),
            Pos::new(3, 5),
            Pos::new(2, 4),
            Pos::new(2, 6),
        ];

        println!("{:?}", valid_moves);
        assert!(valid_moves.iter().all(|m| expected_moves.contains(m)));
    }

    #[test]
    fn test_valid_pawn_moves_white() {
        let game = Game::from_fen_notation("8/8/8/8/8/4p1p1/5P2/8".to_string());
        let pos = Pos::new(6, 5);

        let valid_moves = all_valid_moves_for_pawn(&game, &pos);
        let expected_moves = vec![
            Pos::new(5, 5),
            Pos::new(4, 5),
            Pos::new(5, 4),
            Pos::new(5, 6),
        ];

        println!("{:?}", valid_moves);
        assert!(valid_moves.iter().all(|m| expected_moves.contains(m)));
    }
}
