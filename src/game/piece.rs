use crate::game::{Piece, PieceType, Color};

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece {
            piece_type: piece_type,
            color: color,
            has_moved: false,
        }
    }

    pub fn from_char(c: char) -> Piece {
        let piece_type = match c {
            'P' | 'p' => PieceType::Pawn,
            'R' | 'r' => PieceType::Rook,
            'N' | 'n' => PieceType::Knight,
            'B' | 'b' => PieceType::Bishop,
            'Q' | 'q' => PieceType::Queen,
            'K' | 'k' => PieceType::King,
            _ => panic!("Invalid piece type"),
        };

        let color = match c.is_uppercase() {
            true => Color::White,
            false => Color::Black,
        };

        Piece::new(piece_type, color)
    }

    pub fn char_notation(&self) -> String {
        match self.color {
            Color::Black => self.piece_type.char_notation().to_lowercase(),
            Color::White => self.piece_type.char_notation(),
        }
    }
}

impl PieceType {
    pub fn char_notation(&self) -> String {
        let c = match self {
            PieceType::Pawn => "P",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        };
        String::from(c)
    }
}
