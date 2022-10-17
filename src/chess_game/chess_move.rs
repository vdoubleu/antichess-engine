use crate::chess_game::{ChessMove, Game, Piece, Pos};

impl ChessMove {
    pub fn new(piece: Piece, start_pos: Pos, end_pos: Pos) -> ChessMove {
        ChessMove {
            piece,
            start_pos,
            end_pos,
            promotion: None,
        }
    }

    pub fn new_promo(
        piece: Piece,
        start_pos: Pos,
        end_pos: Pos,
        promotion: Option<Piece>,
    ) -> ChessMove {
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
    pub fn get_xboard_algebraic_notation(&self) -> String {
        let mut notation = String::new();
        notation += self.start_pos.get_algebraic_notation().as_str();
        notation += self.end_pos.get_algebraic_notation().as_str();

        if let Some(promo) = &self.promotion {
            notation += promo.char_notation().to_lowercase().as_str();
        }

        notation
    }

    pub fn from_xboard_algebraic_notation(s: &String, game: &Game) -> ChessMove {
        let start_pos = Pos::from_algebraic_notation(s[0..2].to_string());
        let end_pos = Pos::from_algebraic_notation(s[2..4].to_string());
        let promotion = if s.len() == 5 {
            Some(Piece::from_char(s[4..5].chars().next().unwrap()))
        } else {
            None
        };

        let p = match game.piece_at_pos(&start_pos) {
            Some(&p) => p,
            None => panic!("No piece at start position"),
        };

        ChessMove::new_promo(p, start_pos, end_pos, promotion)
    }
}

impl std::fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_xboard_algebraic_notation())
    }
}
