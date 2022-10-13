mod game;
mod pos;
mod piece;
mod square;
mod color;
mod valid_move_finder;

pub struct Game {
    squares: [[Square; 8]; 8],
    player_turn: Color,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
    has_moved: bool, // if the piece has been moved at least once before, useful for castle and pawn start
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug)]
pub struct Move {
    piece: Piece,
    start_pos: Pos,
    end_pos: Pos,
}