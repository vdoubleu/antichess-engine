struct Game {
    squares: [[Square; 8]; 8],
    player_turn: Color,
}

trait ChessGame {
    // fn pretty_print(&self);
    // fn to_fen_str(&self);
}

impl ChessGame for Game {}

trait Playable {
    // fn move_is_valid(&self, Move);
    // fn make_move(&self, Move);
}

impl Playable for Game {}

struct Pos {
    row: i16,
    col: i16,
}

struct Square {
    pos: Pos,
    piece: Option<Piece>,
}

struct Piece {
    piece_type: PieceType,
    color: Color,
    has_moved: bool, // if the piece has been moved at least once before, useful for castle and pawn start
}

enum Color {
    Black = 0,
    White,
}

enum PieceType {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

struct Move {
    startPos: Pos,
    endPos: Pos,
}

fn main() {}
