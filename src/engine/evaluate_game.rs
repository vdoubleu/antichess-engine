use crate::engine::position_scores::*;

use pleco::{core::CastleType, Board, Piece, PieceType, Player, SQ};

/// This will evaluate the game state of the board and return a score
/// This returns a score from the point of view of the white player. We only
/// need to return a single value since this is a zero sum game. So, a
/// positive score is good for white, and a negative score is good for black.
/// White is maximizing, black is minimizing.
pub fn evaluate(board: &Board) -> f64 {
    if board.checkmate() {
        if board.turn() == Player::White {
            return f64::NEG_INFINITY;
        } else {
            return f64::INFINITY;
        }
    }

    if board.stalemate() {
        return 0.0;
    }

    let mut score = 0.0;

    for (sq, piece) in board.get_piece_locations() {
        if piece == Piece::None {
            continue;
        }

        let piece_score = evaluate_material(&piece.type_of())
            + evaluate_piece_pos(&piece, sq, board.ply() as i64)
            + evaluate_threats(board, &piece, sq);

        if piece.player() == Some(Player::White) {
            score += piece_score;
        } else if piece.player() == Some(Player::Black) {
            score -= piece_score;
        }
    }

    score += evaluate_castle(board);
    score += evaluate_king_protection(board);

    score
}

fn evaluate_king_protection(board: &Board) -> f64 {
    fn eval_player_king_prot(board: &Board, player: Player) -> f64 {
        let squares_near_king = board.piece_bb(player, PieceType::K);
        let player_squares = board.get_occupied_player(player);

        let occupised_squares_near_king = squares_near_king & player_squares;

        let num_occupised_squares_near_king = occupised_squares_near_king.count_bits();

        num_occupised_squares_near_king as f64 * 10.0
    }

    let white_king_prot = eval_player_king_prot(board, Player::White);
    let black_king_prot = eval_player_king_prot(board, Player::Black);

    white_king_prot - black_king_prot
}

fn evaluate_castle(board: &Board) -> f64 {
    let mut score = 0.0;

    if board.can_castle(Player::White, CastleType::KingSide) {
        score += 15.0;
    }

    if board.can_castle(Player::White, CastleType::QueenSide) {
        score += 15.0;
    }

    if board.can_castle(Player::Black, CastleType::KingSide) {
        score -= 15.0;
    }

    if board.can_castle(Player::Black, CastleType::QueenSide) {
        score -= 15.0;
    }

    score
}

/// following regular piece values
fn evaluate_material(piece: &PieceType) -> f64 {
    match piece {
        PieceType::P => 100.0,
        PieceType::N => 320.0,
        PieceType::B => 330.0,
        PieceType::R => 500.0,
        PieceType::Q => 900.0,
        PieceType::K => 0.0,
        _ => 0.0,
    }
}

/// evalutes the positions of the knights
/// The closer to the middle the knight is, the better
fn evaluate_piece_pos(piece: &Piece, sq: SQ, turns: i64) -> f64 {
    if let Some(p) = piece.player() {
        match piece.type_of() {
            PieceType::P => pawn_position_score(sq, p),
            PieceType::N => knight_position_score(sq, p),
            PieceType::B => bishop_position_score(sq, p),
            PieceType::R => rook_position_score(sq, p, turns),
            PieceType::Q => queen_position_score(sq, p, turns),
            PieceType::K => king_position_score(sq, p, turns),
            _ => 0.0,
        }
    } else {
        0.0
    }
}

/// evaluates threats to either side
fn evaluate_threats(game: &Board, piece: &Piece, pos: SQ) -> f64 {
    fn threat_score_calc(piece_type: PieceType) -> f64 {
        match piece_type {
            PieceType::P => 50.0,
            PieceType::N => 160.0,
            PieceType::B => 170.0,
            PieceType::R => 250.0,
            PieceType::Q => 450.0,
            PieceType::K => 1000.0,
            _ => 0.0,
        }
    }

    let piece_player = match piece.player() {
        Some(p) => p,
        None => return 0.0,
    };

    let moving_squares = game.attacks_from(piece.type_of(), pos, piece_player);

    let piece_types_to_check = vec![
        PieceType::P,
        PieceType::N,
        PieceType::B,
        PieceType::R,
        PieceType::Q,
        PieceType::K,
    ];

    let mut piece_score = 0.0;

    for piece_type in piece_types_to_check {
        let their_piece_bb = game.piece_bb(piece_player.other_player(), piece_type);

        let piece_threat_bb = their_piece_bb & moving_squares;

        let piece_threat_count = piece_threat_bb.count_bits();

        // support is not nearly as useful, since it'll probs def be dead anyways
        let piece_threat_score = piece_threat_count as f64 * threat_score_calc(piece_type);

        piece_score += piece_threat_score;
    }

    piece_score
}

#[cfg(test)]
mod evaluate_tests {
    use super::*;

    #[test]
    fn test_starting_eval() {
        let game = Board::start_pos();
        let score = evaluate(&game);
        assert!(score.abs() < 1.0);
    }

    #[test]
    fn test_start_material_eval_even() {
        let game = Board::start_pos();

        let mut score = 0.0;
        let mut piece_count = 0;
        for pos_ind in 0..64 {
            let pos = SQ(pos_ind);
            match game.piece_at_sq(pos) {
                Piece::None => (),
                piece => {
                    if piece.player() == Some(Player::White) {
                        score += evaluate_material(&piece.type_of());
                    } else if piece.player() == Some(Player::Black) {
                        score -= evaluate_material(&piece.type_of());
                    }
                    piece_count += 1;
                }
            }
        }

        assert!(piece_count == 32);
        assert!(score.abs() < 1.0);
    }

    #[test]
    fn test_eval_threat() {
        let game = Board::from_fen("6k1/4QNpp/2p5/7P/8/6n1/3KP3/2B3BR b - - 0 1").unwrap();
        let pos = SQ(22);
        let piece = game.piece_at_sq(pos).clone();

        let score = evaluate_threats(&game, &piece, pos);

        println!("score: {}", score);
        assert!((score - 350.0).abs() < 0.1);
    }
}
