use crate::chess_game::valid_move_finder::*;
use crate::chess_game::{Color, Game, Piece, PieceType, Pos};

use crate::engine::position_scores::*;

use std::collections::HashMap;

struct EvalStore {
    black_support: HashMap<(usize, usize), usize>,
    white_support: HashMap<(usize, usize), usize>,
    black_threat: HashMap<(usize, usize), usize>,
    white_threat: HashMap<(usize, usize), usize>,
}

/// This will evaluate the game state of the board and return a score
/// This returns a score from the point of view of the white player. We only
/// need to return a single value since this is a zero sum game. So, a
/// positive score is good for white, and a negative score is good for black.
/// White is maximizing, black is minimizing.
pub fn evaluate(game: &Game) -> f64 {
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

    if let Some(winner) = game.game_winner() {
        if winner == Color::White {
            return f64::INFINITY;
        } else {
            return f64::NEG_INFINITY;
        }
    }

    let mut score = 0.0;
    let mut eval_store = EvalStore {
        black_support: HashMap::new(),
        white_support: HashMap::new(),
        black_threat: HashMap::new(),
        white_threat: HashMap::new(),
    };

    for (piece, pos) in game.get_all_pieces() {
        let piece_color = piece.color;

        let piece_score = evaluate_material(&piece)
            + evaluate_piece_pos(&piece, &pos, &game)
            + evaluate_threats_and_support(game, &piece, &pos, &mut eval_store);

        if piece_color == Color::White {
            score += piece_score;
        } else {
            score -= piece_score;
        }
    }

    score
}

fn valid_moves_for_piece_at_pos(game: &Game, piece: PieceType, pos: &Pos) -> Vec<Pos> {
    match piece {
        PieceType::Pawn => all_pawn_moves(game, pos),
        PieceType::Knight => all_knight_moves(game, pos),
        PieceType::Bishop => all_bishop_moves(game, pos, false),
        PieceType::Rook => all_rook_moves(game, pos, false),
        PieceType::Queen => all_queen_moves(game, pos),
        PieceType::King => all_king_moves(game, pos, false),
    }
}

/// following regular piece values
fn evaluate_material(piece: &Piece) -> f64 {
    match piece.piece_type {
        PieceType::Pawn => 1.0,
        PieceType::Knight => 3.2,
        PieceType::Bishop => 3.3,
        PieceType::Rook => 5.0,
        PieceType::Queen => 9.0,
        PieceType::King => 0.0,
    }
}

/// evalutes the positions of the knights
/// The closer to the middle the knight is, the better
fn evaluate_piece_pos(piece: &Piece, pos: &Pos, game: &Game) -> f64 {
    match piece.piece_type {
        PieceType::Pawn => pawn_position_score(pos, piece.color),
        PieceType::Knight => knight_position_score(pos, piece.color),
        PieceType::Bishop => bishop_position_score(pos, piece.color),
        PieceType::Rook => rook_position_score(pos, piece.color, game.turn_counter),
        PieceType::Queen => queen_position_score(pos, piece.color, game.turn_counter),
        PieceType::King => king_position_score(pos, piece.color, game.turn_counter),
    }
}

/// evaluates threats to either side
fn evaluate_threats_and_support(
    game: &Game,
    piece: &Piece,
    pos: &Pos,
    eval_store: &mut EvalStore,
) -> f64 {
    fn threat_score_calc(piece_type: PieceType) -> f64 {
        match piece_type {
            PieceType::Pawn => 0.5,
            PieceType::Knight => 1.6,
            PieceType::Bishop => 1.7,
            PieceType::Rook => 2.5,
            PieceType::Queen => 4.5,
            PieceType::King => 10.0,
        }
    }

    let mut black_score = 0.0;
    let mut white_score = 0.0;

    for end_pos in valid_moves_for_piece_at_pos(game, piece.piece_type, pos) {
        // if we are attacking a square
        if let Some(target_piece) = game.piece_at_pos(&end_pos) {
            // and the target square has an opposing piece (aka we are attacking an opposing piece)
            if piece.color != target_piece.color {
                // we are threatening this much value
                let target_threat_score = threat_score_calc(target_piece.piece_type);
                let my_threat_score = threat_score_calc(piece.piece_type);
                // the more value a given piece is threatening, the more valuable it is
                // i.e. if a single pawn threatens a queen and a rook, it is more valuable than if
                // threatens just a single pawn
                if piece.color == Color::White {
                    let threat_count = *eval_store.white_threat.get(&pos.get_tuple()).unwrap_or(&0);
                    let new_threat_count = threat_count + 1;
                    eval_store
                        .white_threat
                        .insert(pos.get_tuple(), new_threat_count);
                    white_score +=
                        (target_threat_score / my_threat_score) * (new_threat_count as f64 / 2.0);
                } else {
                    let threat_count = *eval_store.black_threat.get(&pos.get_tuple()).unwrap_or(&0);
                    let new_threat_count = threat_count + 1;
                    eval_store
                        .black_threat
                        .insert(pos.get_tuple(), new_threat_count);
                    black_score +=
                        (target_threat_score / my_threat_score) * (new_threat_count as f64 / 2.0);
                }
            } else {
                // you are supporting a piece
                // each time you support one of your pieces, each additional support is worth less and less
                if piece.color == Color::White {
                    let support_count = *eval_store
                        .white_support
                        .get(&end_pos.get_tuple())
                        .unwrap_or(&0);
                    let new_support_count = support_count + 1;
                    eval_store
                        .white_support
                        .insert(end_pos.get_tuple(), new_support_count);
                    white_score += 1.0 / new_support_count as f64;
                } else {
                    let support_count = *eval_store
                        .black_support
                        .get(&end_pos.get_tuple())
                        .unwrap_or(&0);
                    let new_support_count = support_count + 1;
                    eval_store
                        .black_support
                        .insert(end_pos.get_tuple(), new_support_count);
                    black_score += 1.0 / new_support_count as f64;
                }
            }
        }
    }

    white_score - black_score
}

#[cfg(test)]
mod evaluate_tests {
    use super::*;

    #[test]
    fn test_starting_eval() {
        let game = Game::new_starting_game();
        let score = evaluate(&game);
        assert!(score.abs() < 0.0001);
    }
}
