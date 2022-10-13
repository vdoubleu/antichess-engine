use std::fmt;

struct Game {
    squares: [[Square; 8]; 8],
    player_turn: Color,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_string = String::from(format!("Turn: {}\n", self.player_turn));
      
        let divider = "-----------------\n";

        out_string += divider;

        for r in 0..8 {
          out_string += "|";
          for c in 0..8 {
            out_string += self.squares[r][c].char_notation().as_str();
            out_string += "|";
          }
          out_string += "\n";
          out_string += divider;
        }
      
        write!(f, "{}", out_string)
    }
}

impl Game {
    fn new() -> Game {
        let mut new_board = [[Square::new(); 8]; 8];

        for r in 0..8 {
            for c in 0..8 {
                new_board[r][c].pos.row = r;
                new_board[r][c].pos.col = c;
            }
        }

        Game {
            player_turn: Color::White,
            squares: new_board,
        }
    }

    fn add_piece(&mut self, piece: PieceType, color: Color, pos: Pos) -> &mut Self {
        self.squares[pos.row][pos.col].piece = Some(Piece::new(piece, color));

      self
    }

    fn empty_board(&mut self) -> &mut Self {
        for r in 0..8 {
            for c in 0..8 {
                self.squares[r][c].piece = None;
            }
        }

        self
    }

    fn set_starting_pos(&mut self) -> &mut Self {
        for c in 0..8 {
          self.add_piece(PieceType::Pawn, Color::Black, Pos::new(1, c))
            .add_piece(PieceType::Pawn, Color::White, Pos::new(6, c));
        }

        self.add_piece(PieceType::Rook, Color::Black, Pos::new(0, 0))
          .add_piece(PieceType::Rook, Color::White, Pos::new(7, 0))
          .add_piece(PieceType::Knight, Color::Black, Pos::new(0, 1))
          .add_piece(PieceType::Knight, Color::White, Pos::new(7, 1))
          .add_piece(PieceType::Bishop, Color::Black, Pos::new(0, 2))
          .add_piece(PieceType::Bishop, Color::White, Pos::new(7, 2))
          .add_piece(PieceType::Queen, Color::Black, Pos::new(0, 3))
          .add_piece(PieceType::Queen, Color::White, Pos::new(7, 3))
          .add_piece(PieceType::King, Color::Black, Pos::new(0, 4))
          .add_piece(PieceType::King, Color::White, Pos::new(7, 4))
          .add_piece(PieceType::Bishop, Color::Black, Pos::new(0, 5))
          .add_piece(PieceType::Bishop, Color::White, Pos::new(7, 5))
          .add_piece(PieceType::Knight, Color::Black, Pos::new(0, 6))
          .add_piece(PieceType::Knight, Color::White, Pos::new(7, 6))
          .add_piece(PieceType::Rook, Color::Black, Pos::new(0, 7))
          .add_piece(PieceType::Rook, Color::White, Pos::new(7, 7));
        
        self
    }
}

trait Playable {
    // fn move_is_valid(&self, Move);
    // fn make_move(&self, Move);
}

impl Playable for Game {}

#[derive(Copy, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn default() -> Pos {
        Pos { row: 0, col: 0 }
    }
  
    fn new(r: usize, c: usize) -> Pos {
        Pos { row: r, col: c }
    }
}

#[derive(Copy, Clone)]
struct Square {
    pos: Pos,
    piece: Option<Piece>,
}

impl Square {
    fn new() -> Square {
        Square {
            pos: Pos::default(),
            piece: None,
        }
    }
  
    fn char_notation(&self) -> String {
      match self.piece {
        Some(p) => p.char_notation(),
        None => String::from(" "),
      }
    }
}

trait ChessSquare {}

impl ChessSquare for Square {}

#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: Color,
    has_moved: bool, // if the piece has been moved at least once before, useful for castle and pawn start
}

impl Piece {
  fn new(piece_type: PieceType, color: Color) -> Piece {
    Piece {
      piece_type: piece_type,
      color: color,
      has_moved: false,
    }
  }
  
  fn char_notation(&self) -> String {
    match self.color {
      Color::Black => self.piece_type.char_notation().to_lowercase(),
      Color::White => self.piece_type.char_notation(),
    }
  }
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
  fn char_notation(&self) -> String {
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

struct Move {
    startPos: Pos,
    endPos: Pos,
}

fn main() {
    let mut game = Game::new();
    println!("{}", game);

    game.set_starting_pos();

    println!("{}", game);
}
