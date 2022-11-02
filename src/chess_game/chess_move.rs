use crate::chess_game::pos::PosExt;
use crate::chess_game::{ChessMove, Game, Piece, PieceType, Pos};

impl ChessMove {
    /// Creates a ChessMove
    /// If you want to create a chess move that includes a promotion, use `ChessMove::new_promo()`
    pub fn new(start_pos: Pos, end_pos: Pos, promotion: Option<PieceType>) -> ChessMove {
        ChessMove {
            start_pos,
            end_pos,
            promotion,
        }
    }

    /// xboard notation is a little different from standard algebraic notation
    /// it takes the first coordinate as the start position and the second as the end position
    /// so for example, e2e4 is the move e2 to e4
    pub fn get_in_xboard_algebraic_notation(&self) -> String {
        let mut notation = String::new();
        notation += self.start_pos.to_alg_notation().as_str();
        notation += self.end_pos.to_alg_notation().as_str();

        if let Some(promo) = &self.promotion {
            notation += promo
                .char_notation()
                .to_lowercase()
                .collect::<String>()
                .as_str();
        }

        notation
    }

    /// Creates a ChessMove from xboard algebraic notation
    pub fn from_xboard_algebraic_notation(s: &String, game: &Game) -> ChessMove {
        let start_pos = Pos::from_alg_notation(&s[0..2]);
        let end_pos = Pos::from_alg_notation(&s[2..4]);
        let promotion = if s.len() == 5 {
            Some(Piece::from_char(s[4..5].chars().next().unwrap()).piece_type)
        } else {
            None
        };

        ChessMove::new(start_pos, end_pos, promotion)
    }

    pub fn is_en_passant(&self, game: &Game) -> bool {
        let (to_row, to_col) = self.end_pos.to_row_col_as_i8();
        let (from_row, from_col) = self.start_pos.to_row_col_as_i8();
        let moving_piece = game.get_piece(self.start_pos).unwrap();

        if let Some(en_passant_pos) = game.en_passant_pos {
            let (_, ep_col) = en_passant_pos.to_row_col_as_i8();
            if moving_piece.piece_type == PieceType::Pawn
                && (to_col - from_col).abs() == 1
                && game.board[self.end_pos].is_none()
                && ep_col == to_col
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl std::fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_in_xboard_algebraic_notation())
    }
}

impl PartialEq for ChessMove {
    fn eq(&self, other: &Self) -> bool {
        self.start_pos == other.start_pos
            && self.end_pos == other.end_pos
            && self.promotion == other.promotion
    }
}
