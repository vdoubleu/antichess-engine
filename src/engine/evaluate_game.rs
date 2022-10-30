use crate::chess_game::{ChessMove, Color, Game};

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

    f64::NEG_INFINITY
}
