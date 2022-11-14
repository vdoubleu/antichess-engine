use crate::chess_game::{Color, Piece, PieceType};
use crate::error::ChessError;

use anyhow::Result;

impl Piece {
    /// Creates a new Piece with a given color
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }

    /// Takes a character, and returns the corresponding piece
    /// It will take into account the case of the letter to determine the color
    pub fn from_char(c: char) -> Result<Piece> {
        let piece_type = match c {
            'P' | 'p' => PieceType::Pawn,
            'R' | 'r' => PieceType::Rook,
            'N' | 'n' => PieceType::Knight,
            'B' | 'b' => PieceType::Bishop,
            'Q' | 'q' => PieceType::Queen,
            'K' | 'k' => PieceType::King,
            _ => return Err(ChessError::InvalidPieceChar(c).into()),
        };

        let color = match c.is_uppercase() {
            true => Color::White,
            false => Color::Black,
        };

        Ok(Piece::new(piece_type, color))
    }

    /// Takes a piece, and returns the corresponding character
    /// Black pieces are lowercase, and white pieces are uppercase (following standard chess notation)
    pub fn char_notation(&self) -> char {
        match self.color {
            Color::Black => self
                .piece_type
                .char_notation()
                .to_lowercase()
                .collect::<Vec<char>>()[0],
            Color::White => self.piece_type.char_notation(),
        }
    }

    pub fn unicode_char(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::Pawn => '♟',
                PieceType::Rook => '♜',
                PieceType::Knight => '♞',
                PieceType::Bishop => '♝',
                PieceType::Queen => '♛',
                PieceType::King => '♚',
            },
            Color::Black => match self.piece_type {
                PieceType::Pawn => '♙',
                PieceType::Rook => '♖',
                PieceType::Knight => '♘',
                PieceType::Bishop => '♗',
                PieceType::Queen => '♕',
                PieceType::King => '♔',
            },
        }
    }
}

impl PieceType {
    pub fn char_notation(&self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

#[cfg(test)]
mod piece_tests {
    use super::*;

    #[test]
    fn test_from_char() -> Result<()> {
        let piece = Piece::from_char('P')?;
        assert_eq!(piece.piece_type, PieceType::Pawn);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('p')?;
        assert_eq!(piece.piece_type, PieceType::Pawn);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('R')?;
        assert_eq!(piece.piece_type, PieceType::Rook);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('r')?;
        assert_eq!(piece.piece_type, PieceType::Rook);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('N')?;
        assert_eq!(piece.piece_type, PieceType::Knight);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('n')?;
        assert_eq!(piece.piece_type, PieceType::Knight);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('B')?;
        assert_eq!(piece.piece_type, PieceType::Bishop);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('b')?;
        assert_eq!(piece.piece_type, PieceType::Bishop);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('Q')?;
        assert_eq!(piece.piece_type, PieceType::Queen);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('q')?;
        assert_eq!(piece.piece_type, PieceType::Queen);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('K')?;
        assert_eq!(piece.piece_type, PieceType::King);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('k')?;
        assert_eq!(piece.piece_type, PieceType::King);
        assert_eq!(piece.color, Color::Black);

        Ok(())
    }

    #[test]
    fn test_char_notation() {
        let piece = Piece::new(PieceType::Pawn, Color::White);
        assert_eq!(piece.char_notation(), 'P');

        let piece = Piece::new(PieceType::Pawn, Color::Black);
        assert_eq!(piece.char_notation(), 'p');

        let piece = Piece::new(PieceType::Rook, Color::White);
        assert_eq!(piece.char_notation(), 'R');

        let piece = Piece::new(PieceType::Rook, Color::Black);
        assert_eq!(piece.char_notation(), 'r');

        let piece = Piece::new(PieceType::Knight, Color::White);
        assert_eq!(piece.char_notation(), 'N');

        let piece = Piece::new(PieceType::Knight, Color::Black);
        assert_eq!(piece.char_notation(), 'n');

        let piece = Piece::new(PieceType::Bishop, Color::White);
        assert_eq!(piece.char_notation(), 'B');

        let piece = Piece::new(PieceType::Bishop, Color::Black);
        assert_eq!(piece.char_notation(), 'b');

        let piece = Piece::new(PieceType::Queen, Color::White);
        assert_eq!(piece.char_notation(), 'Q');

        let piece = Piece::new(PieceType::Queen, Color::Black);
        assert_eq!(piece.char_notation(), 'q');

        let piece = Piece::new(PieceType::King, Color::White);
        assert_eq!(piece.char_notation(), 'K');

        let piece = Piece::new(PieceType::King, Color::Black);
        assert_eq!(piece.char_notation(), 'k');
    }
}
