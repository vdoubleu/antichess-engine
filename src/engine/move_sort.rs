use crate::chess_game::{ChessMove, Color, Game, Piece, PieceType};
use crate::engine::store::AlphaBetaStore;

pub fn sort_moves(
    game: &Game,
    store: &AlphaBetaStore,
    move_list: &Vec<ChessMove>,
) -> Vec<ChessMove> {
    if move_list.is_empty() {
        return vec![];
    }

    let has_captures = move_list[0].is_direct_capture(game);

    // put moves that capture pieces first, and in order of the value of the piece being captured
    // sort_captures(game, move_list);
    let mut scored_moves: Vec<(&ChessMove, f64)> = move_list.iter().map(|m| (m, 0.0)).collect();

    if has_captures {
        score_captures(game, &mut scored_moves);
    }

    score_pv(store, &mut scored_moves);

    // sort moves
    scored_moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    scored_moves.reverse();

    scored_moves.iter().map(|m| *m.0).collect()
}

fn score_pv(store: &AlphaBetaStore, scored_moves: &mut [(&ChessMove, f64)]) {
    let pv_len = store.pv.len();
    for (m, score) in scored_moves.iter_mut() {
        for (ind, pv_move) in store.pv.iter().enumerate() {
            if &pv_move == m {
                *score += 1000.0 + (pv_len - ind) as f64 * 10.0;
            }
        }
    }
}

fn score_captures(game: &Game, move_list: &mut [(&ChessMove, f64)]) {
    fn eval_cost(piece_type: PieceType) -> f64 {
        match piece_type {
            PieceType::Pawn => 1.0,
            PieceType::Knight => 3.0,
            PieceType::Bishop => 3.0,
            PieceType::Rook => 5.0,
            PieceType::Queen => 9.0,
            PieceType::King => 15.0,
        }
    }

    for (m, score) in move_list.iter_mut() {
        let attacking_piece: PieceType = game
            .get_piece(m.start_pos)
            .unwrap_or_else(|| Piece::new(PieceType::Pawn, Color::White))
            .piece_type;

        let victim_piece: PieceType = game
            .get_piece(m.end_pos)
            .unwrap_or_else(|| Piece::new(PieceType::Pawn, Color::White))
            .piece_type;

        // ratio of two pieces
        *score += eval_cost(victim_piece) / eval_cost(attacking_piece);
    }
}

#[cfg(test)]
mod sort_tests {
    use super::*;

    #[test]
    fn test_sort() {
        let game = Game::from_fen_notation("8/8/3p4/2r3k1/4N3/2n3q1/8/8");

        let mut move_list = vec![
            ChessMove::from_xboard_algebraic_notation("e4d6"),
            ChessMove::from_xboard_algebraic_notation("e4g3"),
            ChessMove::from_xboard_algebraic_notation("e4g5"),
            ChessMove::from_xboard_algebraic_notation("e4c3"),
            ChessMove::from_xboard_algebraic_notation("e4c5"),
        ];

        let store = AlphaBetaStore::new();

        move_list = sort_moves(&game, &store, &move_list);

        assert_eq!(
            move_list[0],
            ChessMove::from_xboard_algebraic_notation("e4g5")
        );
        assert_eq!(
            move_list[1],
            ChessMove::from_xboard_algebraic_notation("e4g3")
        );
        assert_eq!(
            move_list[2],
            ChessMove::from_xboard_algebraic_notation("e4c5")
        );
        assert_eq!(
            move_list[3],
            ChessMove::from_xboard_algebraic_notation("e4c3")
        );
        assert_eq!(
            move_list[4],
            ChessMove::from_xboard_algebraic_notation("e4d6")
        );
    }
}
