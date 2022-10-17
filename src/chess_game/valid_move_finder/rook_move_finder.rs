use crate::chess_game::valid_move_finder::move_within_board_bounds;
use crate::chess_game::{Game, PieceType, Pos};

// the no piece check should be set to false in most cases. It's only used when getting all valid
// moves for the queen because the queen can move like a rook or a bishop
pub fn all_valid_moves_for_rook(game: &Game, pos: &Pos, no_piece_check: bool) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let row = pos.row;
    let col = pos.col;

    let piece = match game.piece_at(row, col) {
        Some(p) => p,
        None => return valid_moves,
    };

    if !no_piece_check && piece.piece_type != PieceType::Rook {
        return valid_moves;
    }

    let color = piece.color;

    let rook_move_directions: Vec<(i16, i16)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (row_dir, col_dir) in rook_move_directions {
        let mut row = row as i16;
        let mut col = col as i16;

        loop {
            row += row_dir;
            col += col_dir;

            if !move_within_board_bounds(row, col) {
                break;
            }

            let m = Pos::new(row as usize, col as usize);

            if game.is_empty_square(&m) {
                valid_moves.push(m);
            } else if game.has_piece_with_color(&m, color.opposite()) {
                valid_moves.push(m);
                break;
            } else {
                break;
            }
        }
    }

    valid_moves
}
