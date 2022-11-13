use crate::chess_game::pos::PosExt;
use crate::chess_game::{ChessMove, Game, Piece, PieceType, Pos};
use crate::error::ChessError;

impl ChessMove {
    /// Creates a ChessMove
    /// If you want to create a chess move that includes a promotion, use `ChessMove::new_promo()`
    pub fn new(start_pos: Pos, end_pos: Pos, promotion: Option<PieceType>) -> ChessMove {
        ChessMove {
            start_pos,
            end_pos,
            promotion,
            is_null_move: false,
        }
    }

    pub fn new_null_move() -> ChessMove {
        ChessMove {
            start_pos: 0,
            end_pos: 0,
            promotion: None,
            is_null_move: true,
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
    pub fn from_xboard_algebraic_notation(s: &str) -> Result<ChessMove, ChessError> {
        let start_pos = Pos::from_alg_notation(&s[0..2]);
        let end_pos = Pos::from_alg_notation(&s[2..4]);
        let promotion = if s.len() == 5 {
            Some(Piece::from_char(s[4..5].chars().next().unwrap())?.piece_type)
        } else {
            None
        };

        Ok(ChessMove::new(start_pos, end_pos, promotion))
    }

    pub fn is_en_passant(&self, game: &Game) -> bool {
        let to_col = self.end_pos.col() as i8;
        let from_col = self.start_pos.col() as i8;
        if let Some(moving_piece) = game.get_piece(self.start_pos) {
            if moving_piece.piece_type != PieceType::Pawn {
                return false;
            }
        }

        if let Some(en_passant_pos) = game.en_passant_pos {
            let ep_col = en_passant_pos.col() as i8;
            (to_col - from_col).abs() == 1 && game.board[self.end_pos].is_none() && ep_col == to_col
        } else {
            false
        }
    }

    pub fn is_direct_capture(&self, game: &Game) -> bool {
        if let Some(end_piece) = game.get_piece(self.end_pos) {
            if let Some(start_piece) = game.get_piece(self.start_pos) {
                return start_piece.color != end_piece.color;
            }
        }

        false
    }

    #[allow(dead_code)]
    pub fn is_castle(&self, game: &Game) -> bool {
        if let Some(start_piece) = game.get_piece(self.start_pos) {
            if start_piece.piece_type == PieceType::King {
                let start_col = self.start_pos.col() as i8;
                let end_col = self.end_pos.col() as i8;
                return (start_col - end_col).abs() == 2;
            }
        }

        false
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
