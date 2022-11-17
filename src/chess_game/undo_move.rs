use crate::chess_game::pos::PosExt;
use crate::chess_game::{ChessMove, Game, Piece, Pos, UndoMove};

impl UndoMove {
    pub fn new(game: &Game, chess_move: &ChessMove) -> Self {
        if chess_move.is_null_move {
            return UndoMove {
                start_pos: chess_move.start_pos,
                end_pos: chess_move.end_pos,
                captured_piece: None,
                en_passant_pos: game.en_passant_pos,
                promotion: None,
                castle_availability_before_move: game.castle_availability,
                is_null_move: true,
                turns_since_take_or_pawn_move: game.turns_since_take_or_pawn_move,
            };
        }

        let captured_piece: Option<(Piece, Pos)> =
            if chess_move.is_en_passant(game) && game.board[chess_move.end_pos].is_none() {
                let (_, to_col) = chess_move.end_pos.to_row_col();
                let (ep_row, _) = game.en_passant_pos.unwrap().to_row_col();
                let cap_piece_pos = Pos::new(ep_row, to_col);

                game.board[cap_piece_pos].map(|piece| (piece, cap_piece_pos))
            } else {
                game.board[chess_move.end_pos].map(|piece| (piece, chess_move.end_pos))
            };

        UndoMove {
            start_pos: chess_move.start_pos,
            end_pos: chess_move.end_pos,

            captured_piece,
            en_passant_pos: game.en_passant_pos,

            promotion: chess_move.promotion,

            castle_availability_before_move: game.castle_availability,

            is_null_move: false,
            turns_since_take_or_pawn_move: game.turns_since_take_or_pawn_move,
        }
    }
}
