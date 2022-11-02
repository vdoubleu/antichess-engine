use crate::chess_game::{Color, Pos};

/// more points the closer it is to promotion
pub fn pawn_position_score(pos: &Pos, color: Color) -> f64 {
    let score = [
        [6, 7, 7, 7, 7, 7, 7, 6],
        [5, 6, 6, 6, 6, 6, 6, 5],
        [4, 5, 5, 5, 5, 5, 5, 4],
        [3, 4, 4, 4, 4, 4, 4, 3],
        [2, 3, 3, 3, 3, 3, 3, 2],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [0, 1, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let row = if color == Color::White { 7 - pos.row } else { pos.row };
    let col = pos.col;

    (score[row][col] as f64) / 7.0
}

/// more points the closer it is to the center
pub fn knight_position_score(pos: &Pos, color: Color) -> f64 {
    let score = [
        [0, 1, 1, 1, 1, 1, 1, 0],
        [1, 2, 5, 3, 3, 5, 2, 1],
        [2, 3, 5, 5, 5, 5, 3, 2],
        [2, 3, 5, 5, 5, 5, 3, 2],
        [2, 3, 5, 5, 5, 5, 3, 2],
        [2, 3, 5, 5, 5, 5, 3, 2],
        [1, 2, 3, 3, 3, 3, 2, 1],
        [0, -3, 0, 0, 0, 0, -3, 0],
    ];

    let row = if color == Color::White { 7 - pos.row } else { pos.row };
    let col = pos.col;

    (score[row][col] as f64) / 7.0
}

/// more points if closer to long diagonals
pub fn bishop_position_score(pos: &Pos, color: Color) -> f64 {
    let score = [
        [1, 1, 2, 0, 0, 2, 1, 1],
        [1, 2, 3, 2, 2, 3, 2, 1],
        [2, 3, 4, 5, 5, 4, 3, 2],
        [3, 4, 5, 6, 6, 5, 4, 3],
        [3, 4, 5, 6, 6, 5, 4, 3],
        [2, 3, 4, 5, 5, 4, 3, 2],
        [1, 2, 3, 4, 4, 3, 2, 1],
        [2, 1, 0, 3, 3, 0, 1, 2],
    ];

    let row = if color == Color::White { 7 - pos.row } else { pos.row };
    let col = pos.col;

    (score[row][col] as f64) / 7.0
}

/// more points if closer to safety in the back
pub fn king_position_score(pos: &Pos, color: Color, turn_counter: i64) -> f64 {
    let score = [
        [-1, -2, -2, -2, -2, -2, -2, -1],
        [-1, -3, -3, -3, -3, -3, -3, -1],
        [-1, -2, -3, -3, -3, -3, -2, -1],
        [-1, -2, -3, -3, -3, -3, -2, -1],
        [-1, -2, -2, -3, -3, -2, -2, -1],
        [0, 0, -2, -1, -1, -2, 0, 0],
        [0, 0, 0, -1, -1, 0, 0, 0],
        [6, 6, 5, 2, 5, 6, 7, 6],
    ];

    let row = if color == Color::White { 7 - pos.row } else { pos.row };
    let col = pos.col;

    let scale = if turn_counter < 5 {
        1.0
    } else if turn_counter < 25 {
        -0.1 * (turn_counter as f64) + 1.5
    } else {
        -1.0
    };

    scale * (score[row][col] as f64) / 7.0
}

pub fn rook_position_score(pos: &Pos, color: Color, turn_counter: i64) -> f64 {
    let score_start = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 0, 0, 0, 0, 0, 0, 2],
        [2, 2, 3, 3, 3, 3, 2, 2],
    ];

    let score_end = [
        [0, 1, 1, 2, 2, 1, 1, 0],
        [1, 2, 5, 3, 3, 5, 2, 1],
        [1, 3, 5, 5, 5, 5, 3, 1],
        [2, 3, 5, 5, 5, 5, 3, 2],
        [2, 3, 5, 5, 5, 5, 3, 2],
        [1, 3, 5, 5, 5, 5, 3, 1],
        [1, 2, 3, 3, 3, 3, 2, 1],
        [0, 1, 1, 2, 2, 1, 1, 0],
    ];

    let row = if color == Color::White { 7 - pos.row } else { pos.row };
    let col = pos.col;

    let scale = if turn_counter < 5 {
        1.0
    } else {
        (-0.1 * (turn_counter as f64) + 1.5).max(0.0)
    };

    ((scale * (score_start[row][col] as f64)) / 7.0 + (scale * (score_end[row][col] as f64)) / 7.0) / 2.0
}

pub fn queen_position_score(pos: &Pos, color: Color, turn_counter: i64) -> f64 {
    let score_start = [
        [5, 5, 5, 5, 5, 5, 5, 5],
        [3, 5, 5, 5, 5, 5, 5, 3],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 0, 0, 0, 0, 0, 0, 2],
        [2, 2, 3, 3, 3, 3, 2, 2],
    ];

    let score_end = [
        [0, 1, 1, 2, 2, 1, 1, 0],
        [1, 2, 5, 3, 3, 5, 2, 1],
        [1, 3, 5, 5, 5, 5, 3, 1],
        [2, 3, 5, 5, 5, 5, 3, 2],
        [2, 3, 5, 5, 5, 5, 3, 2],
        [1, 3, 5, 5, 5, 5, 3, 1],
        [1, 2, 3, 3, 3, 3, 2, 1],
        [0, 1, 1, 2, 2, 1, 1, 0],
    ];

    let row = if color == Color::White { 7 - pos.row } else { pos.row };
    let col = pos.col;

    let scale = if turn_counter < 5 {
        1.0
    } else {
        (-0.1 * (turn_counter as f64) + 1.5).max(0.0)
    };

    ((scale * (score_start[row][col] as f64)) / 7.0 + (scale * (score_end[row][col] as f64)) / 7.0) / 2.0
}
