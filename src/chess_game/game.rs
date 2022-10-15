use crate::chess_game::valid_move_finder::*;
use crate::chess_game::{ChessMove, Color, Game, Piece, PieceType, Pos, Square};
use std::fmt;

impl Game {
    pub fn new() -> Game {
        let mut new_board = [[Square::new(); 8]; 8];

        for (r_ind, r) in new_board.iter_mut().enumerate() {
            for (c_ind, s) in r.iter_mut().enumerate() {
                s.pos.row = r_ind;
                s.pos.col = c_ind;
            }
        }

        Game {
            player_turn: Color::White,
            squares: new_board,
        }
    }

    pub fn new_starting_game() -> Game {
        Game::from_fen_notation(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"))
    }

    pub fn from_fen_notation(fen_str: String) -> Game {
        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut game = Game::new();

        for c in fen_str.chars() {
            if c == '/' {
                row += 1;
                col = 0;
            } else if c.is_ascii_digit() {
                col += c.to_digit(10).unwrap() as usize;
            } else {
                let piece = Piece::from_char(c);
                game.square_at_mut(row, col).piece = Some(piece);
                col += 1;
            }
        }

        game
    }

    pub fn get_player_turn(&self) -> Color {
        self.player_turn
    }

    pub fn add_piece(&mut self, piece: PieceType, color: Color, pos: &Pos) -> &mut Self {
        self.square_at_pos_mut(pos).piece = Some(Piece::new(piece, color));

        self
    }

    pub fn empty_board(&mut self) -> &mut Self {
        for r in 0..8 {
            for c in 0..8 {
                self.square_at_mut(r, c).piece = None;
            }
        }

        self
    }

    pub fn set_starting_pos(&mut self) -> &mut Self {
        self.empty_board()
            .set_with_fen_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

        self
    }

    pub fn get_fen_notation(&self) -> String {
        let mut fen_string = String::new();

        for r in 0..8 {
            let mut empty_count = 0;
            for c in 0..8 {
                if let Some(piece) = &self.piece_at(r, c) {
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

    pub fn set_with_fen_notation(&mut self, fen_str: String) -> &mut Self {
        let mut row: usize = 0;
        let mut col: usize = 0;

        self.empty_board();

        for c in fen_str.chars() {
            if c == '/' {
                row += 1;
                col = 0;
            } else if c.is_ascii_digit() {
                col += c.to_digit(10).unwrap() as usize;
            } else {
                let piece = Piece::from_char(c);
                self.square_at_mut(row, col).piece = Some(piece);
                col += 1;
            }
        }

        self
    }

    pub fn is_empty_square(&self, pos: &Pos) -> bool {
        if pos.row > 7 || pos.col > 7 {
            return false;
        }

        self.piece_at_pos(pos).is_none()
    }

    pub fn has_piece_with_color(&self, pos: &Pos, color: Color) -> bool {
        if pos.row > 7 || pos.col > 7 {
            return false;
        }

        match &self.piece_at_pos(pos) {
            Some(piece) => piece.color == color,
            None => false,
        }
    }

    pub fn valid_moves_for_piece(
        &self,
        pos: &Pos,
        only_check_currently_attacking: bool,
    ) -> Vec<Pos> {
        if let Some(piece) = &self.piece_at_pos(pos) {
            match piece.piece_type {
                PieceType::Pawn => {
                    all_valid_moves_for_pawn(self, pos, only_check_currently_attacking)
                }
                PieceType::Knight => all_valid_moves_for_knight(self, pos),
                PieceType::Bishop => all_valid_moves_for_bishop(self, pos, false),
                PieceType::Rook => all_valid_moves_for_rook(self, pos, false),
                PieceType::Queen => all_valid_moves_for_queen(self, pos),
                PieceType::King => {
                    all_valid_moves_for_king(self, pos, only_check_currently_attacking)
                }
            }
        } else {
            panic!("no piece at pos");
        }
    }

    pub fn all_valid_moves_for_color(
        &self,
        color: Color,
        only_check_currently_attacking: bool,
    ) -> Vec<ChessMove> {
        let mut all_valid_moves: Vec<ChessMove> = Vec::new();

        for r in 0..8 {
            for c in 0..8 {
                let cur_pos = Pos::new(r, c);
                if self.has_piece_with_color(&cur_pos, color) {
                    let valid_moves =
                        self.valid_moves_for_piece(&cur_pos, only_check_currently_attacking);
                    for valid_move in valid_moves {
                        all_valid_moves.push(ChessMove::new(
                            *self.piece_at(r, c).unwrap(),
                            cur_pos,
                            valid_move,
                        ));
                    }
                }
            }
        }

        all_valid_moves
    }

    pub fn all_valid_moves_for_color_that_take(&self, color: Color) -> Vec<ChessMove> {
        let mut all_valid_moves: Vec<ChessMove> = Vec::new();

        for r in 0..8 {
            for c in 0..8 {
                let cur_pos = Pos::new(r, c);
                if self.has_piece_with_color(&cur_pos, color) {
                    let valid_moves = self.valid_moves_for_piece(&cur_pos, true);
                    for valid_move in valid_moves {
                        if self.has_piece_with_color(&valid_move, color.opposite()) {
                            all_valid_moves.push(ChessMove::new(
                                *self.piece_at(r, c).unwrap(),
                                cur_pos,
                                valid_move,
                            ));
                        }
                    }
                }
            }
        }

        all_valid_moves
    }

    pub fn move_is_valid(&self, from: &Pos, to: &Pos) -> bool {
        let mut valid = false;

        if let Some(piece) = &self.squares[from.row][from.col].piece {
            if piece.color == self.player_turn {
                let valid_moves = self.valid_moves_for_piece(from, false);
                for m in valid_moves {
                    if m == *to {
                        valid = true;
                        break;
                    }
                }
            }
        }

        valid
    }

    pub fn make_move(&mut self, user_move: &ChessMove) -> &mut Self {
        self.move_piece(
            &user_move.start_pos,
            &user_move.end_pos,
            user_move.promotion,
        )
    }

    pub fn move_piece(&mut self, from: &Pos, to: &Pos, promotion: Option<Piece>) -> &mut Self {
        // check move is valid
        if self.move_is_valid(from, to) {
            // move piece
            self.square_at_pos_mut(to).piece = self.squares[from.row][from.col].piece.take();

            // set has moved
            self.piece_at_pos_mut(to).unwrap().has_moved = true;

            // check for promotion
            let p = self.piece_at_pos(to).unwrap();
            if p.piece_type == PieceType::Pawn
                && ((p.color == Color::White && to.row == 0)
                    || (p.color == Color::Black && to.row == 7))
            {
                if let Some(promotion_piece) = promotion {
                    let new_promo = Piece {
                        piece_type: promotion_piece.piece_type,
                        color: p.color,
                        has_moved: true,
                    };

                    self.square_at_pos_mut(to).piece = Some(new_promo);
                } else {
                    panic!("no promotion piece provided");
                }
            }

            // check for castling
            let p = self.piece_at_pos(to).unwrap();
            if p.piece_type == PieceType::King {
                if from.col == 4 && to.col == 6 {
                    self.relocate_piece(&Pos::new(to.row, 7), &Pos::new(to.row, 5));
                } else if from.col == 4 && to.col == 2 {
                    self.relocate_piece(&Pos::new(to.row, 0), &Pos::new(to.row, 3));
                }
            }

            // change player turn
            self.player_turn = match self.player_turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        } else {
            panic!("invalid move");
        }

        self
    }

    // like move_piece but doesn't check if move is valid or any other fancy stuff
    fn relocate_piece(&mut self, from: &Pos, to: &Pos) -> &mut Self {
        self.square_at_pos_mut(to).piece = self.squares[from.row][from.col].piece.take();
        self
    }

    // checkes if pos is being attacked by color
    pub fn square_attacked_by_color(&self, pos: &Pos, color: Color) -> bool {
        let valid_move_for_color = self.all_valid_moves_for_color(color, true);

        for m in valid_move_for_color {
            if m.end_pos == *pos {
                return true;
            }
        }

        false
    }

    pub fn piece_at(&self, r: usize, c: usize) -> Option<&Piece> {
        self.squares[r][c].piece.as_ref()
    }

    pub fn piece_at_mut(&mut self, r: usize, c: usize) -> Option<&mut Piece> {
        self.squares[r][c].piece.as_mut()
    }

    pub fn piece_at_pos(&self, pos: &Pos) -> Option<&Piece> {
        self.squares[pos.row][pos.col].piece.as_ref()
    }

    pub fn piece_at_pos_mut(&mut self, pos: &Pos) -> Option<&mut Piece> {
        self.squares[pos.row][pos.col].piece.as_mut()
    }

    pub fn square_at(&self, r: usize, c: usize) -> &Square {
        &self.squares[r][c]
    }

    pub fn square_at_mut(&mut self, r: usize, c: usize) -> &mut Square {
        &mut self.squares[r][c]
    }

    pub fn square_at_pos(&self, pos: &Pos) -> &Square {
        &self.squares[pos.row][pos.col]
    }

    pub fn square_at_pos_mut(&mut self, pos: &Pos) -> &mut Square {
        &mut self.squares[pos.row][pos.col]
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_string = format!("Turn: {}\n", self.player_turn);

        let divider = " -----------------\n";

        out_string += divider;

        for r in 0..8 {
            out_string += (8 - r).to_string().as_str();
            out_string += "|";
            for c in 0..8 {
                out_string += self.squares[r][c].char_notation().as_str();
                out_string += "|";
            }
            out_string += "\n";
            out_string += divider;
        }

        out_string += "  a b c d e f g h\n";

        write!(f, "{}", out_string)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
