use crate::chess_game::pos::PosExt;
use crate::chess_game::{Game, Pos};

// the no piece check should be set to false in most cases. It's only used when getting all valid
// moves for the queen because the queen can move like a rook or a bishop
pub fn all_valid_moves_for_rook(game: &Game, pos: Pos) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let piece = match game.get_piece(pos) {
        Some(p) => p,
        None => return valid_moves,
    };

    let color = piece.color;

    let rook_move_directions: Vec<i8> = vec![1, -1, 10, -10];

    for move_dist in rook_move_directions {
        let mut curr_pos = pos;

        loop {
            curr_pos = ((curr_pos as i8) + move_dist) as usize;

            if !curr_pos.is_on_board() {
                break;
            }

            if game.board[curr_pos].is_none() {
                valid_moves.push(curr_pos);
            } else if game.has_piece_with_color(curr_pos, color.opposite()) {
                valid_moves.push(curr_pos);
                break;
            } else {
                break;
            }
        }
    }

    valid_moves
}
