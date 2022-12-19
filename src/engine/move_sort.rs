use crate::engine::store::AlphaBetaStore;
use pleco::{BitMove, Board, PieceType};

pub fn sort_moves(board: &Board, store: &AlphaBetaStore, move_list: &Vec<BitMove>) -> Vec<BitMove> {
    if move_list.is_empty() {
        return vec![];
    }

    let has_captures = move_list[0].is_capture();

    // put moves that capture pieces first, and in order of the value of the piece being captured
    let mut scored_moves: Vec<(&BitMove, f64)> = move_list.iter().map(|m| (m, 0.0)).collect();

    if has_captures {
        score_captures(board, &mut scored_moves);
    }

    score_pv(store, &mut scored_moves);
    score_tt(store, board, &mut scored_moves);

    // sort moves
    scored_moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    scored_moves.reverse();

    scored_moves.iter().map(|m| *m.0).collect()
}

fn score_pv(store: &AlphaBetaStore, scored_moves: &mut [(&BitMove, f64)]) {
    let pv_len = store.pv.len();
    for (m, score) in scored_moves.iter_mut() {
        for (ind, pv_move) in store.pv.iter().enumerate() {
            if &pv_move == m {
                *score += 10000.0 + (pv_len - ind) as f64 * 10.0;
            }
        }
    }
}

fn score_tt(store: &AlphaBetaStore, board: &Board, scored_moves: &mut [(&BitMove, f64)]) {
    let transpo = store.get_transposition(board);

    if transpo.is_none() {
        return;
    }

    let (transpo_entry, is_curr_ply) = transpo.unwrap();
    if let Some(transpo_move) = transpo_entry.chess_move {
        for (m, score) in scored_moves.iter_mut() {
            if &transpo_move == *m {
                if is_curr_ply {
                    *score += 1000.0;
                } else {
                    *score += 500.0;
                }
            }
        }
    }
}

fn score_captures(board: &Board, move_list: &mut [(&BitMove, f64)]) {
    fn eval_cost(piece_type: PieceType) -> f64 {
        match piece_type {
            PieceType::P => 1.0,
            PieceType::N => 3.0,
            PieceType::B => 3.0,
            PieceType::R => 5.0,
            PieceType::Q => 9.0,
            PieceType::K => 15.0,
            _ => 0.0,
        }
    }

    for (m, score) in move_list.iter_mut() {
        let attacking_piece: PieceType = board.piece_at_sq(m.get_src()).type_of();

        let victim_piece: PieceType = board.piece_at_sq(m.get_dest()).type_of();

        // ratio of two pieces
        *score += eval_cost(victim_piece) / eval_cost(attacking_piece);
    }
}

#[cfg(test)]
mod sort_tests {
    use super::*;

    use pleco::{File, Rank, SQ};

    use anyhow::{bail, Result};

    #[test]
    fn test_sort() -> Result<()> {
        let game = match Board::from_fen("K7/8/3p4/2r5/4N3/2n3q1/8/k7 w - - 0 1") {
            Ok(g) => g,
            Err(e) => bail!("Failed to parse FEN: {:?}", e),
        };

        let mut move_list = vec![
            BitMove::make_capture(SQ::make(File::E, Rank::R4), SQ::make(File::D, Rank::R6)),
            BitMove::make_capture(SQ::make(File::E, Rank::R4), SQ::make(File::G, Rank::R3)),
            BitMove::make_capture(SQ::make(File::E, Rank::R4), SQ::make(File::C, Rank::R3)),
            BitMove::make_capture(SQ::make(File::E, Rank::R4), SQ::make(File::C, Rank::R5)),
        ];

        let store = AlphaBetaStore::new();

        move_list = sort_moves(&game, &store, &move_list);

        let expectec_move_list = vec![
            BitMove::make_capture(SQ::make(File::E, Rank::R4), SQ::make(File::G, Rank::R3)),
            BitMove::make_capture(SQ::make(File::E, Rank::R4), SQ::make(File::C, Rank::R5)),
            BitMove::make_capture(SQ::make(File::E, Rank::R4), SQ::make(File::C, Rank::R3)),
            BitMove::make_capture(SQ::make(File::E, Rank::R4), SQ::make(File::D, Rank::R6)),
        ];

        assert_eq!(move_list, expectec_move_list);

        Ok(())
    }
}
