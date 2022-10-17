use crate::chess_game::valid_move_finder::move_within_board_bounds;
use crate::chess_game::{Game, PieceType, Pos};

pub fn all_valid_moves_for_knight(game: &Game, pos: &Pos) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let row = pos.row;
    let col = pos.col;

    let piece = match game.piece_at(row, col) {
        Some(p) => p,
        None => return valid_moves,
    };

    if piece.piece_type != PieceType::Knight {
        return valid_moves;
    }

    let color = piece.color;

    let moves: Vec<(i16, i16)> = vec![
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];

    for (row_offset, col_offset) in moves {
        let new_row = row as i16 + row_offset;
        let new_col = col as i16 + col_offset;
        if !move_within_board_bounds(new_row, new_col) {
            continue;
        }

        let new_pos = Pos::new(new_row as usize, new_col as usize);
        if game.is_empty_square(&new_pos) || game.has_piece_with_color(&new_pos, color.opposite()) {
            valid_moves.push(new_pos);
        }
    }

    valid_moves
}

#[cfg(test)]
mod knight_tests {
    use super::*;

    #[test]
    fn test_valid_moves() {
        let game = Game::from_fen_notation("8/8/8/8/3N4/8/8/8".to_string());
        let knight_pos = Pos::from_algebraic_notation("d4".to_string());

        let valid_moves = all_valid_moves_for_knight(&game, &knight_pos);

        assert_eq!(valid_moves.len(), 8);

        let expected_moves = vec![
            Pos::from_algebraic_notation("c6".to_string()),
            Pos::from_algebraic_notation("e6".to_string()),
            Pos::from_algebraic_notation("f5".to_string()),
            Pos::from_algebraic_notation("f3".to_string()),
            Pos::from_algebraic_notation("e2".to_string()),
            Pos::from_algebraic_notation("c2".to_string()),
            Pos::from_algebraic_notation("b3".to_string()),
            Pos::from_algebraic_notation("b5".to_string()),
        ];

        for expected_move in expected_moves {
            assert!(valid_moves.contains(&expected_move));
        }
    }
}
