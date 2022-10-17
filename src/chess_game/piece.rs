use crate::chess_game::{Color, Piece, PieceType};

impl Piece {
    /// Creates a new Piece with a given color
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece {
            piece_type,
            color,
            last_moved: -1,
            last_moved_from: None,
        }
    }

    /// Takes a character, and returns t he corresponding piece 
    /// It will take into account the case of the letter to determine the color
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

    /// Takes a piece, and returns the corresponding character
    /// Black pieces are lowercase, and white pieces are uppercase (following standard chess notation)
    pub fn char_notation(&self) -> String {
        match self.color {
            Color::Black => self.piece_type.char_notation().to_lowercase(),
            Color::White => self.piece_type.char_notation(),
        }
    }

    /// Returns true if the piece has moved at least once before during this game
    pub fn has_moved(&self) -> bool {
        self.last_moved != -1
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
