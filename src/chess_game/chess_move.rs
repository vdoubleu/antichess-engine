use crate::chess_game::{ChessMove, Piece, Pos};

impl ChessMove {
    pub fn new(piece: Piece, start_pos: Pos, end_pos: Pos) -> ChessMove {
        ChessMove {
            piece,
            start_pos,
            end_pos,
            promotion: None,
        }
    }

    pub fn new_promo(piece: Piece, start_pos: Pos, end_pos: Pos, promotion: Option<Piece>) -> ChessMove {
        ChessMove {
            piece,
            start_pos,
            end_pos,
            promotion,
        }
    }

    // xboard notation is a little different from standard algebraic notation
    // it takes the first coordinate as the start position and the second as the end position
    // so for example, e2e4 is the move e2 to e4
    pub fn to_xboard_algebraic_notation(&self) -> String {
        let mut notation = String::new();
        notation += self.start_pos.to_algebraic_notation().as_str();
        notation += self.end_pos.to_algebraic_notation().as_str();

        if let Some(promo) = &self.promotion {
            notation += promo.char_notation().to_lowercase().as_str();
        }

        notation
    }
}

impl std::fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_xboard_algebraic_notation())
    }
}