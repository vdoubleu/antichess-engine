mod chess_move;
mod color;
mod game;
mod piece;
mod pos;
mod square;
mod valid_move_finder;

#[derive(Clone)]
pub struct Game {
    squares: [[Square; 8]; 8],
    player_turn: Color,
    turn_counter: i64,
    winner: Option<Color>,
}

#[derive(Copy, Clone, Debug)]
pub struct Pos {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone)]
pub struct Square {
    pos: Pos,
    piece: Option<Piece>,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
    /// when the piece last moved, it's -1 if it hasn't moved yet
    last_moved: i64,
    last_moved_from: Option<Pos>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
    // piece: Piece,
    start_pos: Pos,
    end_pos: Pos,
    promotion: Option<Piece>,
}
