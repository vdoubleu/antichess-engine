use crate::chess_game::{ChessMove, Game, Piece, PieceType, Pos};

impl ChessMove {
    /// Creates a ChessMove
    /// If you want to create a chess move that includes a promotion, use `ChessMove::new_promo()`
    pub fn new(
        piece: Piece,
        start_pos: Pos,
        end_pos: Pos,
        captured_piece: Option<(Piece, Pos)>,
    ) -> ChessMove {
        ChessMove {
            piece,
            start_pos,
            end_pos,
            captured_piece,
            promotion: None,
        }
    }

    /// Pretty much the same as just the regular new function, but will take a game context to be
    /// able to figure out what pieces were captured
    pub fn new_with_context(
        start_pos: Pos,
        end_pos: Pos,
        game: &Game,
        promotion: Option<Piece>,
    ) -> ChessMove {
        let curr_piece = game.piece_at_pos(&start_pos).cloned().unwrap();

        let captured_piece = game.piece_at_pos(&end_pos).cloned();

        let captured_piece_data: Option<(Piece, Pos)> =
            if let Some(the_captured_piece) = captured_piece {
                Some((the_captured_piece, end_pos))
            } else if curr_piece.piece_type == PieceType::Pawn && start_pos.col != end_pos.col {
                let en_passant_pos = Pos::new(start_pos.row, end_pos.col);
                let en_passant_piece = game.piece_at_pos(&en_passant_pos).cloned().unwrap();
                Some((en_passant_piece, en_passant_pos))
            } else {
                None
            };

        ChessMove {
            piece: curr_piece,
            start_pos,
            end_pos,
            captured_piece: captured_piece_data,
            promotion,
        }
    }

    /// Creates a new ChessMove with a promotion
    pub fn new_promo(
        piece: Piece,
        start_pos: Pos,
        end_pos: Pos,
        captured_piece: Option<(Piece, Pos)>,
        promotion: Option<Piece>,
    ) -> ChessMove {
        ChessMove {
            piece,
            start_pos,
            end_pos,
            captured_piece,
            promotion,
        }
    }

    /// xboard notation is a little different from standard algebraic notation
    /// it takes the first coordinate as the start position and the second as the end position
    /// so for example, e2e4 is the move e2 to e4
    pub fn get_xboard_algebraic_notation(&self) -> String {
        let mut notation = String::new();
        notation += self.start_pos.get_algebraic_notation().as_str();
        notation += self.end_pos.get_algebraic_notation().as_str();

        if let Some(promo) = &self.promotion {
            notation += promo.char_notation().to_lowercase().as_str();
        }

        notation
    }

    /// Creates a ChessMove from xboard algebraic notation
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

        let captured_piece = game.piece_at_pos(&end_pos).map(|&p| (p, end_pos));

        ChessMove::new_promo(p, start_pos, end_pos, captured_piece, promotion)
    }
}

impl std::fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_xboard_algebraic_notation())
    }
}
