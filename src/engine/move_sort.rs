use crate::chess_game::{ChessMove, Game, PieceType};
use std::cmp::Ordering;

pub fn sort_moves(game: &Game, move_list: &mut [ChessMove]) {
    // put moves that capture pieces first, and in order of the value of the piece being captured
    sort_captures(game, move_list);
}

fn sort_captures(game: &Game, move_list: &mut [ChessMove]) {
    // sort captures by the value of the piece being captured

    fn eval_cost(piece_type: PieceType) -> f64 {
        match piece_type {
            PieceType::Pawn => 1.0,
            PieceType::Knight => 3.0,
            PieceType::Bishop => 3.0,
            PieceType::Rook => 5.0,
            PieceType::Queen => 9.0,
            PieceType::King => f64::INFINITY,
        }
    }

    // TODO sort pieces by agressor and victim
    // TODO skip if not a capture

    move_list.sort_by(|a, b| {
        let a_piece = game.get_piece(a.end_pos);
        let b_piece = game.get_piece(b.end_pos);
        if a_piece.is_none() && b_piece.is_none() {
            return Ordering::Equal;
        } else if a_piece.is_none() {
            return Ordering::Greater;
        } else if b_piece.is_none() {
            return Ordering::Less;
        }
        let a_piece = a_piece.unwrap();
        let b_piece = b_piece.unwrap();
        let a_value = eval_cost(a_piece.piece_type);
        let b_value = eval_cost(b_piece.piece_type);
        a_value.partial_cmp(&b_value).unwrap()
    });

    move_list.reverse();
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

        sort_moves(&game, &mut move_list);

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
