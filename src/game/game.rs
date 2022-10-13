use std::fmt;
use crate::game::{Game, Square, Piece, PieceType, Color, Pos, Move};
use crate::game::valid_move_finder::*;

impl Game {
    pub fn new() -> Game {
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

    pub fn add_piece(&mut self, piece: PieceType, color: Color, pos: Pos) -> &mut Self {
        self.squares[pos.row][pos.col].piece = Some(Piece::new(piece, color));

        self
    }

    pub fn empty_board(&mut self) -> &mut Self {
        for r in 0..8 {
            for c in 0..8 {
                self.squares[r][c].piece = None;
            }
        }

        self
    }

    pub fn set_starting_pos(&mut self) -> &mut Self {
        self.empty_board()
            .from_fen_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

        self
    }

    pub fn to_fen_notation(&self) -> String {
        let mut fen_string = String::new();

        for r in 0..8 {
            let mut empty_count = 0;
            for c in 0..8 {
                if let Some(piece) = &self.squares[r][c].piece {
                    if empty_count > 0 {
                        fen_string += empty_count.to_string().as_str();
                        empty_count = 0;
                    }
                    fen_string += piece.char_notation().as_str();
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen_string += empty_count.to_string().as_str();
            }
            if r < 7 {
                fen_string += "/";
            }
        }

        fen_string
    }

    pub fn from_fen_notation(&mut self, fen_str: String) -> &mut Self {
        let mut row: usize = 0;
        let mut col: usize = 0;

        self.empty_board();

        for c in fen_str.chars() {
            if c == '/' {
                row += 1;
                col = 0;
            } else if c.is_digit(10) {
                col += c.to_digit(10).unwrap() as usize;
            } else {
                let piece = Piece::from_char(c);
                self.squares[row][col].piece = Some(piece);
                col += 1;
            }
        }

        self
    }

    pub fn is_empty_square(&self, pos: &Pos) -> bool {
        if pos.row > 7 || pos.col > 7 {
            return false;
        }

        self.squares[pos.row][pos.col].piece.is_none()
    }

    pub fn has_piece_with_color(&self, pos: &Pos, color: Color) -> bool {
        if pos.row > 7 || pos.col > 7 {
            return false;
        }

        match &self.squares[pos.row][pos.col].piece {
            Some(piece) => piece.color == color,
            None => false,
        }
    }

    pub fn valid_moves_for_piece(&self, pos: &Pos) -> Vec<Pos> {
        if let Some(piece) = &self.squares[pos.row][pos.col].piece {
            match piece.piece_type {
                PieceType::Pawn => all_valid_moves_for_pawn(self, &pos),
                PieceType::Knight => all_valid_moves_for_knight(self, &pos),
                PieceType::Bishop => all_valid_moves_for_bishop(self, &pos, false),
                PieceType::Rook => all_valid_moves_for_rook(self, &pos, false),
                PieceType::Queen => all_valid_moves_for_queen(self, &pos),
                PieceType::King => all_valid_moves_for_king(self, &pos),
            }
        } else {
            panic!("no piece at pos");
        }
    }

    pub fn all_valid_moves_for_color(&self, color: Color) -> Vec<Move> {
        let mut all_valid_moves: Vec<Move> = Vec::new();

        for r in 0..8 {
            for c in 0..8 {
                let cur_pos = Pos::new(r, c);
                if self.has_piece_with_color(&cur_pos, color) {
                    let valid_moves = self.valid_moves_for_piece(&cur_pos);
                    for valid_move in valid_moves {
                        all_valid_moves.push(Move {
                            piece: self.squares[r][c].piece.unwrap(),
                            start_pos: cur_pos,
                            end_pos: valid_move,
                        });
                    }
                }
            }
        }

        all_valid_moves
    }

    pub fn all_valid_moves_for_color_that_take(&self, color: Color) -> Vec<Move> {
        let mut all_valid_moves: Vec<Move> = Vec::new();

        for r in 0..8 {
            for c in 0..8 {
                let cur_pos = Pos::new(r, c);
                if self.has_piece_with_color(&cur_pos, color) {
                    let valid_moves = self.valid_moves_for_piece(&cur_pos);
                    for valid_move in valid_moves {
                        if self.has_piece_with_color(&valid_move, color.opposite()) {
                            all_valid_moves.push(Move {
                                piece: self.squares[r][c].piece.unwrap(),
                                start_pos: cur_pos,
                                end_pos: valid_move,
                            });
                        }
                    }
                }
            }
        }

        all_valid_moves
    }

    pub fn move_is_valid(&self, from: Pos, to: Pos) -> bool {
        let mut valid = false;

        if let Some(piece) = &self.squares[from.row][from.col].piece {
            if piece.color == self.player_turn {
                let valid_moves = self.valid_moves_for_piece(&from);

                for m in valid_moves {
                    if m == to {
                        valid = true;
                        break;
                    }
                }
            }
        }

        valid
    }

    pub fn move_piece(&mut self, from: Pos, to: Pos, promotion: Option<Piece>) -> &mut Self {
        // check move is valid
        if self.move_is_valid(from, to) {
            // move piece
            self.squares[to.row][to.col].piece = self.squares[from.row][from.col].piece.take();
            
            self.squares[to.row][to.col].piece.unwrap().has_moved = true;

            // change player turn
            self.player_turn = match self.player_turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };

            // check for promotion
            let p = self.squares[to.row][to.col].piece.unwrap();
            if p.piece_type == PieceType::Pawn {
                if p.color == Color::White && to.row == 0 {
                    self.squares[to.row][to.col].piece = promotion;
                } else if p.color == Color::Black && to.row == 7 {
                    self.squares[to.row][to.col].piece = promotion;
                } 
            }
        }

        self
    }
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
