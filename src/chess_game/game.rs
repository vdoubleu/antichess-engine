use crate::chess_game::valid_move_finder::{
    all_bishop_moves, all_king_moves, all_knight_moves, all_pawn_moves, all_queen_moves,
    all_rook_moves,
};
use crate::chess_game::{ChessMove, Color, Game, Piece, PieceType, Pos, Square};
use std::fmt;

impl Game {
    /// Creates a new empty board
    /// I would advise against using this since I'm not sure why you would want an empty board.
    /// You are probably looking for `Game::new_starting_game()` or `Game::from_fen_notation(fen_str)`
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
            turn_counter: 0,
            winner: None,
        }
    }

    /// Creates a new game with the starting board
    pub fn new_starting_game() -> Game {
        Game::from_fen_notation(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"))
    }

    /// Creates a new game from a FEN string
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

    /// Returns the color of the current player
    pub fn get_player_turn(&self) -> Color {
        self.player_turn
    }

    /// Adds a piece at a given square. Not sure why exactly you would want this...
    /// This is mostly for internal use when creating a new game
    pub fn add_piece(&mut self, piece: PieceType, color: Color, pos: &Pos) -> &mut Self {
        self.square_at_pos_mut(pos).piece = Some(Piece::new(piece, color));

        self
    }

    /// Empties the board
    pub fn empty_board(&mut self) -> &mut Self {
        for r in 0..8 {
            for c in 0..8 {
                self.square_at_mut(r, c).piece = None;
            }
        }

        self
    }

    /// Empties the board and resets the board to the starting position
    pub fn set_starting_pos(&mut self) -> &mut Self {
        self.empty_board()
            .set_with_fen_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

        self
    }

    /// Gets the fen notation for the current board state
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

    /// Resets the board to the given fen notation
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

    /// Checks if the given position on the board is empty
    pub fn is_empty_square(&self, pos: &Pos) -> bool {
        if pos.row > 7 || pos.col > 7 {
            return false;
        }

        self.piece_at_pos(pos).is_none()
    }

    /// Checks if the given position on the board has a piece of the given color
    pub fn has_piece_with_color(&self, pos: &Pos, color: Color) -> bool {
        if pos.row > 7 || pos.col > 7 {
            return false;
        }

        match &self.piece_at_pos(pos) {
            Some(piece) => piece.color == color,
            None => false,
        }
    }

    /// Returns all the valid moves for the piece at the given position
    pub fn valid_moves_for_piece(
        &self,
        pos: &Pos,
    ) -> Vec<Pos> {
        self.valid_moves_for_piece_impl(pos, false)
    }

    fn valid_moves_for_piece_impl(
        &self,
        pos: &Pos,
        only_check_currently_attacking: bool,
    ) -> Vec<Pos> {
        if let Some(piece) = &self.piece_at_pos(pos) {
            match piece.piece_type {
                PieceType::Pawn => all_pawn_moves(self, pos),
                PieceType::Knight => all_knight_moves(self, pos),
                PieceType::Bishop => all_bishop_moves(self, pos, false),
                PieceType::Rook => all_rook_moves(self, pos, false),
                PieceType::Queen => all_queen_moves(self, pos),
                PieceType::King => all_king_moves(self, pos, only_check_currently_attacking),
            }
        } else {
            panic!("no piece at pos");
        }
    }

    /// Returns a vector of all the valid moves for the color
    pub fn all_valid_moves_for_color(
        &self,
        color: Color,
    ) -> Vec<ChessMove> {
        self.all_valid_moves_for_color_impl(color, false)
    }
    
    /// WARNING!! This is an implementation function, you probably don't want to use this.
    /// The function you probably want to use is `all_valid_moves_for_color`.
    /// 
    /// Returns all the valid moves for a color
    /// Generally set only_check_currently_attacking to false. We only set it to 
    /// true when we don't want to include castling (since it causes weird infinite recursions)
    fn all_valid_moves_for_color_impl(
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
                        self.valid_moves_for_piece_impl(&cur_pos, only_check_currently_attacking);
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

    /// Returns all the valid moves that a color can make that take pieces.
    pub fn all_valid_moves_for_color_that_take(&self, color: Color) -> Vec<ChessMove> {
        let mut all_valid_moves: Vec<ChessMove> = Vec::new();

        for r in 0..8 {
            for c in 0..8 {
                let cur_pos = Pos::new(r, c);
                if self.has_piece_with_color(&cur_pos, color) {
                    let valid_moves = self.valid_moves_for_piece_impl(&cur_pos, true);
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

    /// If a winner exists, returns the winner. Otherwise returns None.
    /// If it returns None, then the game is not over.
    pub fn game_winner(&self) -> Option<Color> {
        self.winner
    }

    /// Checks if a move from one position to another is valid
    pub fn move_is_valid(&self, from: &Pos, to: &Pos) -> bool {
        let mut valid = false;

        if let Some(piece) = &self.squares[from.row][from.col].piece {
            if piece.color == self.player_turn {
                let valid_moves = self.valid_moves_for_piece_impl(from, false);
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

    /// Makes the ChessMove
    pub fn make_move(&mut self, user_move: &ChessMove) -> &mut Self {
        self.move_piece(
            &user_move.start_pos,
            &user_move.end_pos,
            user_move.promotion,
        )
    }

    /// Moves a piece, would recommend against using this directly, use make_move instead
    /// since it is a little bit easier to work with.
    pub fn move_piece(&mut self, from: &Pos, to: &Pos, promotion: Option<Piece>) -> &mut Self {
        // check move is valid
        if self.move_is_valid(from, to) {
            // Checks if the player has taken a king and has thus won the game
            if let Some(to_piece) = self.piece_at_pos(to) {
                if to_piece.piece_type == PieceType::King {
                    self.winner = Some(self.player_turn);
                }
            }

            // check for en passant
            // we have to check this before we move the piece
            // because we need to know if the position we are moving to is empty
            let en_passant_p = self.piece_at_pos(from).unwrap();
            if en_passant_p.piece_type == PieceType::Pawn {
                // if move to another col and no piece there, must be en passant
                if from.col != to.col && self.piece_at_pos(to).is_none() {
                    let target_pos_to_take = Pos::new(from.row, to.col);

                    self.square_at_pos_mut(&target_pos_to_take).piece = None;
                }
            }

            // move piece
            self.square_at_pos_mut(to).piece = self.squares[from.row][from.col].piece.take();

            // set has last moved
            self.piece_at_pos_mut(to).unwrap().last_moved = self.turn_counter;
            self.piece_at_pos_mut(to).unwrap().last_moved_from = Some(*from);

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
                        last_moved: self.turn_counter,
                        last_moved_from: Some(*from),
                    };

                    self.square_at_pos_mut(to).piece = Some(new_promo);
                } else {
                    panic!("no promotion piece provided");
                }
            }

            // check for castling
            let p = self.piece_at_pos(to).unwrap();
            if p.piece_type == PieceType::King {
                let (rook_to_pos, rook_from_pos) = if from.col == 4 && to.col == 6 {
                    (Pos::new(to.row, 5), Pos::new(to.row, 7))
                } else if from.col == 4 && to.col == 2 {
                    (Pos::new(to.row, 3), Pos::new(to.row, 0))
                } else {
                    panic!("invalid king or rook move when castling");
                };

                self.relocate_piece(&rook_from_pos, &rook_to_pos);
                self.piece_at_pos_mut(&rook_to_pos).unwrap().last_moved = self.turn_counter;
                self.piece_at_pos_mut(&rook_to_pos).unwrap().last_moved_from = Some(rook_from_pos);
            }

            // change player turn
            self.player_turn = match self.player_turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };

            self.turn_counter += 1;
        } else {
            panic!("invalid move");
        }

        self
    }

    /// Similar to move_piece but doesn't check if move is valid or any other fancy stuff
    fn relocate_piece(&mut self, from: &Pos, to: &Pos) -> &mut Self {
        self.square_at_pos_mut(to).piece = self.squares[from.row][from.col].piece.take();
        self
    }

    // Checks if pos is being attacked by color
    pub fn square_attacked_by_color(&self, pos: &Pos, color: Color) -> bool {
        let valid_move_for_color = self.all_valid_moves_for_color_impl(color, true);

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
