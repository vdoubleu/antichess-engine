use crate::chess_game::pos::PosExt;
use crate::chess_game::{Color, Game, Pos};

// the no piece check should be set to false in most cases. It's only used when getting all valid
// moves for the queen because the queen can move like a rook or a bishop
pub fn all_valid_moves_for_bishop(game: &Game, pos: Pos, color: Color) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let bishop_move_directions: Vec<i8> = vec![9, 11, -9, -11];

    for move_dist in bishop_move_directions {
        let mut curr_pos = pos;
        loop {
            curr_pos = ((curr_pos as i8) + move_dist) as usize;

            if !curr_pos.on_board() {
                break;
            }

            if game.board[curr_pos].is_none() {
                // square is empty
                valid_moves.push(curr_pos);
            } else if game.has_piece_with_color(curr_pos, color.opposite()) {
                // square has opp piece
                valid_moves.push(curr_pos);
                break;
            } else {
                // square has your piece
                break;
            }
        }
    }

    valid_moves
}
