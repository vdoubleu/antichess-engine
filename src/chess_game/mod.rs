pub mod chess_move;
pub mod color;
pub mod game;
pub mod piece;
pub mod pos;
pub mod undo_move;
pub mod valid_move_finder;

pub type Pos = usize;

#[derive(Clone)]
pub struct Game {
    pub board: [Option<Piece>; 120],
    pub player_turn: Color,
    pub turn_counter: i64,
    pub castle_availability: [bool; 4],
    pub en_passant_pos: Option<Pos>,

    pub winner: Option<Color>,

    pub undo_move_history: Vec<UndoMove>,
}

pub enum CastleTypes {
    WhiteKing = 0,
    WhiteQueen = 1,
    BlackKing = 2,
    BlackQueen = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
pub struct ChessMove {
    pub start_pos: Pos,
    pub end_pos: Pos,

    pub promotion: Option<PieceType>,
    pub is_null_move: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct UndoMove {
    pub start_pos: Pos,
    pub end_pos: Pos,

    pub captured_piece: Option<(Piece, Pos)>,
    pub en_passant_pos: Option<Pos>,

    pub promotion: Option<PieceType>,

    pub castle_availability_before_move: [bool; 4],

    pub is_null_move: bool,
}

pub fn promotable_pieces() -> Vec<PieceType> {
    vec![
        PieceType::Queen,
        PieceType::Rook,
        PieceType::Bishop,
        PieceType::Knight,
    ]
}
