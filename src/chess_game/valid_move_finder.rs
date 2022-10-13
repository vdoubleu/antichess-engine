use crate::chess_game::{Color, Game, PieceType, Pos};

fn move_within_board_bounds(row: i16, col: i16) -> bool {
    // assumes board is 8x8
    let r = 0..8;
    r.contains(&row) && r.contains(&col)
}

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
    let has_moved = piece.has_moved;

    if color == Color::Black {
        let in_front = Pos::new(row + 1, col);
        if game.is_empty_square(&in_front) {
            valid_moves.push(in_front);
        }

        // TODO: support en passant

        // take left
        if move_within_board_bounds((row as i16) + 1, (col as i16) - 1) {
            let left = Pos::new(row + 1, col - 1);
            if game.has_piece_with_color(&left, Color::White) {
                valid_moves.push(left);
            }
        }

        // take right
        if move_within_board_bounds((row as i16) + 1, (col as i16) + 1) {
            let right = Pos::new(row + 1, col + 1);
            if game.has_piece_with_color(&right, Color::White) {
                valid_moves.push(right);
            }
        }

        if !has_moved {
            let two_in_front = Pos::new(row + 2, col);
            if game.is_empty_square(&two_in_front) {
                valid_moves.push(two_in_front);
            }
        }
    } else {
        let in_front = Pos::new(row - 1, col);
        if game.is_empty_square(&in_front) {
            valid_moves.push(in_front);
        }

        // take left
        if move_within_board_bounds((row as i16) - 1, (col as i16) - 1) {
            let left = Pos::new(row - 1, col - 1);
            if game.has_piece_with_color(&left, Color::Black) {
                valid_moves.push(left);
            }
        }

        // take right
        if move_within_board_bounds((row as i16) - 1, (col as i16) + 1) {
            let right = Pos::new(row - 1, col + 1);
            if game.has_piece_with_color(&right, Color::Black) {
                valid_moves.push(right);
            }
        }

        if !has_moved {
            let two_in_front = Pos::new(row - 2, col);
            if game.is_empty_square(&two_in_front) {
                valid_moves.push(two_in_front);
            }
        }
    }

    valid_moves
}

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

// the no piece check should be set to false in most cases. It's only used when getting all valid
// moves for the queen because the queen can move like a rook or a bishop
pub fn all_valid_moves_for_bishop(game: &Game, pos: &Pos, no_piece_check: bool) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let row = pos.row;
    let col = pos.col;

    let piece = match game.piece_at(row, col) {
        Some(p) => p,
        None => return valid_moves,
    };

    if !no_piece_check && piece.piece_type != PieceType::Bishop {
        return valid_moves;
    }

    let color = piece.color;

    let bishop_move_directions: Vec<(i16, i16)> = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];

    for (row_dir, col_dir) in bishop_move_directions {
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

pub fn all_valid_moves_for_king(game: &Game, pos: &Pos) -> Vec<Pos> {
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

    // castling
    // check if king has moved
    if piece.has_moved {
        return valid_moves;
    }

    // TODO: implement this
    // check if king is in check
    // if game.is_in_check(color) {
    //     return valid_moves;
    // }

    // check if rooks have moved
    let rook_pos = match color {
        Color::White => vec![Pos::new(0, 0), Pos::new(0, 7)],
        Color::Black => vec![Pos::new(7, 0), Pos::new(7, 7)],
    };

    for pos in rook_pos {
        let piece = match game.piece_at_pos(&pos) {
            Some(p) => p,
            None => continue,
        };

        if piece.has_moved {
            continue;
        }

        // check if there are any pieces between the king and the rook
        let (_king_row, king_col) = match color {
            Color::White => (0, 4),
            Color::Black => (7, 4),
        };

        let (rook_row, rook_col) = (pos.row, pos.col);

        let (start, end) = if rook_col < king_col {
            (rook_col + 1, king_col)
        } else {
            (king_col + 1, rook_col)
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

        // if we get here, then the king can castle
        valid_moves.push(Pos::new(rook_row, end));
    }

    valid_moves
}
