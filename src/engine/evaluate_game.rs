use crate::chess_game::{Color, Game, PieceType};
use std::collections::HashMap;

/// This will evaluate the game state of the board and return a score
/// This returns a score from the point of view of the white player. We only
/// need to return a single value since this is a zero sum game. So, a
/// positive score is good for white, and a negative score is good for black.
pub fn evaluate(game: &Game, color: Color) -> f64 {
    // We will evaluate based on several factors (these factors are based on stockfish):
    // 1. Material
    // 2. Imbalance
    // 3. Pawns
    // 4. Knight
    // 5. Bishop
    // 6. Rook
    // 7. Queen
    // 8. Mobility
    // 9. King Safety
    // 10. Threats
    // 11. Passed Pawns
    // 12. Space
    // 13. Winnable

    evaluate_material(game, color) + evaluate_knight_bishop_positions(game, color)
}

/// following regular piece values
fn evaluate_material(game: &Game, color: Color) -> f64 {
    let mut score = 0.0;

    let piece_value_map = HashMap::from([
        (PieceType::Pawn, 1.0),
        (PieceType::Knight, 3.2),
        (PieceType::Bishop, 3.3),
        (PieceType::Rook, 5.0),
        (PieceType::Queen, 9.0),
        (PieceType::King, 0.0),
    ]);

    for square in game.get_all_squares_of_color(color) {
        let piece = square.piece.unwrap();
        let res = piece_value_map.get(&piece.piece_type);
        if let Some(value) = res {
            score += value;
        }
    }

    score
}

/// evalutes the positions of the knights
/// The closer to the middle the knight is, the better
fn evaluate_knight_bishop_positions(game: &Game, color: Color) -> f64 {
    let mut score = 0.0;

    for square in game.get_all_squares_of_color(color) {
        let piece = square.piece.unwrap();
        if piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Knight {
            let pos = square.pos;
            let row = pos.row;
            let col = pos.col;

            let row_score = if row < 4 {
                row as f64
            } else {
                (7 - row) as f64
            };
            let col_score = if col < 4 {
                col as f64
            } else {
                (7 - col) as f64
            };

            score += row_score + col_score;
        }
    }

    score
}
