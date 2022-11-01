mod chess_move;
mod color;
mod game;
mod piece;
mod pos;
mod square;
pub mod valid_move_finder;

#[derive(Clone)]
pub struct Game {
    pub squares: [[Square; 8]; 8],
    pub player_turn: Color,
    pub turn_counter: i64,
    pub winner: Option<Color>,
}

#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

#[derive(Copy, Clone)]
pub struct Square {
    pub pos: Pos,
    pub piece: Option<Piece>,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    /// when the piece last moved, it's -1 if it hasn't moved yet
    pub last_moved: i64,
    pub last_moved_from: Option<Pos>,
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
    pub piece: Piece,
    pub start_pos: Pos,
    pub end_pos: Pos,
    pub promotion: Option<Piece>,
}
