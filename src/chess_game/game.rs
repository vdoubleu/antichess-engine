use crate::chess_game::pos::{PosExt, BOARD_END, BOARD_START};
use crate::chess_game::valid_move_finder::{
    all_bishop_moves, all_king_moves, all_knight_moves, all_pawn_moves, all_queen_moves,
    all_rook_moves,
};
use crate::chess_game::{
    promotable_pieces, CastleTypes, ChessMove, Color, Game, Piece, PieceType, Pos, UndoMove,
};
use std::fmt;

impl Game {
    /// Creates a new empty board
    /// I would advise against using this since I'm not sure why you would want an empty board.
    /// You are probably looking for `Game::new_starting_game()` or `Game::from_fen_notation(fen_str)`
    pub fn new() -> Game {
        let new_board: [Option<Piece>; 120] = [None; 120];

        Game {
            board: new_board,
            player_turn: Color::White,
            turn_counter: 0,
            castle_availability: [true; 4],
            en_passant_pos: None,
            winner: None,
            undo_move_history: Vec::new(),
        }
    }

    /// Adds a piece at a given square. Not sure why exactly you would want this...
    /// This is mostly for internal use when creating a new game
    pub fn add_piece(&mut self, piece: &Piece, pos: Pos) -> &mut Self {
        self.board[pos] = Some(*piece);
        self
    }

    /// Removes a piece at a given square.
    pub fn remove_piece(&mut self, pos: Pos) -> &mut Self {
        self.board[pos] = None;
        self
    }

    /// Relocates a piece from one square to another
    pub fn relocate_piece(&mut self, start_pos: Pos, end_pos: Pos) -> &mut Self {
        if let Some(piece) = self.board[start_pos] {
            self.board[end_pos] = Some(piece);
            self.board[start_pos] = None;
        } else {
            panic!("No piece at start_pos {}", start_pos);
        }

        self
    }

    pub fn get_piece(&self, pos: Pos) -> Option<Piece> {
        self.board[pos]
    }

    pub fn from_fen_notation(fen_str: &str) -> Game {
        let mut game = Game::new();

        let mut curr_board_pos = 21;
        for c in fen_str.chars() {
            if c == '/' {
                curr_board_pos += 2;
            } else if c.is_ascii_digit() {
                curr_board_pos += c.to_digit(10).unwrap() as usize;
            } else {
                let piece = Piece::from_char(c);
                game.add_piece(&piece, curr_board_pos);
                curr_board_pos += 1;
            }
        }

        game
    }

    /// Creates a new game with the starting board
    pub fn new_starting_game() -> Game {
        Game::from_fen_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    }

    pub fn get_all_piece_pos(&self) -> Vec<Pos> {
        let mut pieces = Vec::new();

        for board_ind in BOARD_START..=BOARD_END {
            if self.board[board_ind].is_some() {
                pieces.push(board_ind);
            }
        }

        pieces
    }

    /// Gets the fen notation for the current board state
    pub fn get_fen_notation(&self) -> String {
        let mut fen_string = String::new();

        for r in 0..8 {
            let mut empty_spaces = 0;

            for c in 0..8 {
                if let Some(piece) = self.board[Pos::new(r, c)] {
                    if empty_spaces > 0 {
                        fen_string.push_str(&empty_spaces.to_string());
                        empty_spaces = 0;
                    }

                    fen_string.push(piece.char_notation());
                } else {
                    empty_spaces += 1;
                }
            }

            if empty_spaces > 0 {
                fen_string.push_str(&empty_spaces.to_string());
            }

            if r != 7 {
                fen_string.push('/');
            }
        }

        fen_string
    }

    /// Checks if the given position on the board has a piece of the given color
    pub fn has_piece_with_color(&self, pos: Pos, color: Color) -> bool {
        assert!(pos.is_on_board());

        match &self.get_piece(pos) {
            Some(piece) => piece.color == color,
            None => false,
        }
    }

    pub fn get_all_pieces(&self) -> Vec<(Piece, Pos)> {
        let mut pieces = Vec::new();

        for board_ind in BOARD_START..=BOARD_END {
            if let Some(piece) = self.board[board_ind] {
                pieces.push((piece, board_ind));
            }
        }

        pieces
    }

    /// Returns all the valid moves for the piece at the given position
    pub fn valid_moves_for_piece(&self, pos: Pos) -> Vec<Pos> {
        self.valid_moves_for_piece_impl(pos, false)
    }

    fn valid_moves_for_piece_impl(
        &self,
        pos: Pos,
        only_check_currently_attacking: bool,
    ) -> Vec<Pos> {
        if let Some(piece) = self.get_piece(pos) {
            match piece.piece_type {
                PieceType::Pawn => all_pawn_moves(self, pos, only_check_currently_attacking),
                PieceType::Knight => all_knight_moves(self, pos),
                PieceType::Bishop => all_bishop_moves(self, pos),
                PieceType::Rook => all_rook_moves(self, pos),
                PieceType::Queen => all_queen_moves(self, pos),
                PieceType::King => all_king_moves(self, pos, only_check_currently_attacking),
            }
        } else {
            panic!("no piece at pos");
        }
    }

    /// Returns a vector of all the valid moves for the color
    /// If there are any moves that take, it will prioritize those by only listing those
    pub fn all_valid_moves_for_color(&self, color: Color) -> Vec<ChessMove> {
        let all_valid_moves = self.all_valid_moves_for_color_impl(color, false);
        let moves_that_take = all_valid_moves
            .iter()
            .filter(|m| self.is_move_that_takes(m, color))
            .copied()
            .collect::<Vec<ChessMove>>();

        if !moves_that_take.is_empty() {
            moves_that_take
        } else {
            all_valid_moves
        }
    }

    pub fn is_move_that_takes(&self, m: &ChessMove, color: Color) -> bool {
        self.has_piece_with_color(m.end_pos, color.opposite()) || m.is_en_passant(self)
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

        for from_pos in self.get_all_piece_pos() {
            let piece = self.get_piece(from_pos).unwrap();
            if piece.color == color {
                let valid_moves =
                    self.valid_moves_for_piece_impl(from_pos, only_check_currently_attacking);

                for to_pos in valid_moves {
                    let to_row = to_pos.row();
                    if piece.piece_type == PieceType::Pawn && (to_row == 1 || to_row == 7) {
                        for promo_piece in promotable_pieces() {
                            all_valid_moves.push(ChessMove::new(
                                from_pos,
                                to_pos,
                                Some(promo_piece),
                            ));
                        }
                    } else {
                        all_valid_moves.push(ChessMove::new(from_pos, to_pos, None));
                    }
                }
            }
        }

        all_valid_moves
    }

    /// Checks if a move from one position to another is valid
    pub fn move_is_valid(&self, from: Pos, to: Pos) -> bool {
        if let Some(piece) = self.board[from] {
            if piece.color == self.player_turn {
                let valid_to_pos = self.valid_moves_for_piece_impl(from, false);
                for m in valid_to_pos {
                    if m == to {
                        return true;
                    }
                }
            } else {
                println!("moving an opponents piece");
            }
        }

        false
    }

    // moves a piece from one position to another
    pub fn move_piece(&mut self, chess_move: &ChessMove) -> &mut Self {
        // check move is valid
        if !self.move_is_valid(chess_move.start_pos, chess_move.end_pos) {
            panic!("Invalid move");
        }

        let move_undoer = UndoMove::new(self, chess_move);
        self.undo_move_history.push(move_undoer);

        let moving_piece_type = self.board[chess_move.start_pos].unwrap().piece_type;
        let (from_row, from_col) = chess_move.start_pos.to_row_col_as_i8();
        let (to_row, to_col) = chess_move.end_pos.to_row_col_as_i8();

        let king_was_taken = if let Some(p) = self.board[chess_move.end_pos] {
            p.piece_type == PieceType::King
        } else {
            false
        };

        // check for en passant
        if chess_move.is_en_passant(self) {
            self.board[self.en_passant_pos.unwrap()] = None;
        }

        // move piece
        self.remove_piece(chess_move.end_pos);
        self.board[chess_move.end_pos] = self.board[chess_move.start_pos].take();

        // check for promotion
        if let Some(promotion) = chess_move.promotion {
            self.board[chess_move.end_pos] = Some(Piece::new(promotion, self.player_turn));
        }

        // moved twice, have to set en_passant_square
        if moving_piece_type == PieceType::Pawn && (to_row - from_row).abs() == 2 {
            self.en_passant_pos = Some(chess_move.end_pos);
        } else {
            self.en_passant_pos = None;
        }

        // set the castle_availability for king
        if moving_piece_type == PieceType::King {
            if self.player_turn == Color::White {
                self.castle_availability[CastleTypes::WhiteKing as usize] = false;
                self.castle_availability[CastleTypes::WhiteQueen as usize] = false;
            } else {
                self.castle_availability[CastleTypes::BlackKing as usize] = false;
                self.castle_availability[CastleTypes::BlackQueen as usize] = false;
            }
        }

        // set the castle_availability for rook
        if moving_piece_type == PieceType::Rook {
            if self.player_turn == Color::White {
                if chess_move.start_pos == Pos::new(0, 0) {
                    self.castle_availability[CastleTypes::WhiteQueen as usize] = false;
                } else if chess_move.start_pos == Pos::new(0, 7) {
                    self.castle_availability[CastleTypes::WhiteKing as usize] = false;
                }
            } else {
                if chess_move.start_pos == Pos::new(7, 0) {
                    self.castle_availability[CastleTypes::BlackQueen as usize] = false;
                } else if chess_move.start_pos == Pos::new(7, 7) {
                    self.castle_availability[CastleTypes::BlackKing as usize] = false;
                }
            }
        }

        // check for castling
        if moving_piece_type == PieceType::King && (from_col - to_col).abs() == 2 {
            let usize_to_row = to_row as usize;
            let (rook_to_pos, rook_from_pos) = if to_col == 6 {
                (Pos::new(usize_to_row, 5), Pos::new(usize_to_row, 7))
            } else if from_col == 4 && to_col == 2 {
                (Pos::new(usize_to_row, 3), Pos::new(usize_to_row, 0))
            } else {
                panic!("invalid king or rook move when castling");
            };

            self.relocate_piece(rook_from_pos, rook_to_pos);

            if self.player_turn == Color::White {
                self.castle_availability[CastleTypes::WhiteKing as usize] = false;
                self.castle_availability[CastleTypes::WhiteQueen as usize] = false;
            } else {
                self.castle_availability[CastleTypes::BlackKing as usize] = false;
                self.castle_availability[CastleTypes::BlackQueen as usize] = false;
            }
        }

        // eval winner
        if king_was_taken {
            self.winner = Some(self.player_turn);
        }

        // change player turn
        self.player_turn = self.player_turn.opposite();
        self.turn_counter += 1;

        self
    }

    pub fn unmove_move(&mut self) -> bool {
        if self.undo_move_history.is_empty() {
            panic!("no moves to undo");
        }

        let undo_move = self.undo_move_history.pop().unwrap();

        self.winner = None; // reset winner, we can't undo and still have a winner
        self.player_turn = self.player_turn.opposite();
        self.turn_counter -= 1;
        self.en_passant_pos = undo_move.en_passant_pos;
        self.castle_availability = undo_move.castle_availability_before_move;

        if self.board[undo_move.end_pos].is_none() {
            println!("undoing a move that didn't move a piece");
            println!("undo_move: {:?}", undo_move);
            println!("self: {}", self);
            return false;
        }

        let undo_piece_type = self.board[undo_move.end_pos].unwrap().piece_type;
        let start_col = undo_move.start_pos.col() as i8;
        let (end_row, end_col) = undo_move.end_pos.to_row_col_as_i8();

        // move piece back
        self.board[undo_move.start_pos] = self.board[undo_move.end_pos].take();

        // unpromote piece
        if undo_move.promotion.is_some() {
            self.board[undo_move.start_pos].unwrap().piece_type = PieceType::Pawn;
        }

        // untake piece
        if let Some((captured_piece, captured_piece_pos)) = undo_move.captured_piece {
            self.add_piece(&captured_piece, captured_piece_pos);
        }

        // uncastle if necessary
        if undo_piece_type == PieceType::King && (start_col - end_col).abs() == 2 {
            let usize_end_row = end_row as usize;
            let (rook_move_start, rook_move_end) = if end_col == 6 {
                (Pos::new(usize_end_row, 5), Pos::new(usize_end_row, 7))
            } else if end_col == 2 {
                (Pos::new(usize_end_row, 3), Pos::new(usize_end_row, 0))
            } else {
                panic!("invalid king or rook move when castling");
            };

            self.relocate_piece(rook_move_start, rook_move_end);
        }

        true
    }

    // Checks if pos is being attacked by color
    pub fn square_attacked_by_color(&self, pos: Pos, color: Color) -> bool {
        let valid_move_for_color = self.all_valid_moves_for_color_impl(color, true);

        for m in valid_move_for_color {
            if m.end_pos == pos {
                return true;
            }
        }

        false
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
                let s = if let Some(piece) = self.get_piece(Pos::new(r, c)) {
                    piece.char_notation().to_string()
                } else {
                    " ".to_string()
                };
                out_string += s.as_str();
                out_string += "|";
            }
            out_string += "\n";
            out_string += divider;
        }

        out_string += "  a b c d e f g h\n";

        write!(f, "{}", out_string)
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;
    use crate::chess_game::ChessMove;

    #[test]
    fn test_create_game() {
        let game = Game::new_starting_game();
        assert_eq!(game.player_turn, Color::White);
        assert_eq!(game.turn_counter, 0);
        assert_eq!(game.winner, None);
        assert_eq!(game.en_passant_pos, None);
        assert_eq!(game.castle_availability, [true, true, true, true]);
        assert_eq!(
            game.get_fen_notation(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_make_move() {
        let mut game = Game::new_starting_game();
        let m1 = ChessMove::new(Pos::new(6, 0), Pos::new(4, 0), None);
        let m2 = ChessMove::new(Pos::new(1, 0), Pos::new(3, 0), None);
        game.move_piece(&m1);
        game.move_piece(&m2);
        assert_eq!(game.player_turn, Color::White);
        assert_eq!(game.turn_counter, 2);
        assert_eq!(game.winner, None);
        assert_eq!(game.en_passant_pos, Some(Pos::new(3, 0)));
        assert_eq!(game.castle_availability, [true, true, true, true]);
        assert_eq!(
            game.get_fen_notation(),
            "rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_unmake_move() {
        let mut game = Game::new_starting_game();
        let m1 = ChessMove::new(Pos::new(6, 0), Pos::new(4, 0), None);
        let m2 = ChessMove::new(Pos::new(1, 0), Pos::new(3, 0), None);

        game.move_piece(&m1);
        game.move_piece(&m2);
        game.unmove_move();
        game.unmove_move();

        assert_eq!(game.player_turn, Color::White);
        assert_eq!(game.turn_counter, 0);
        assert_eq!(game.winner, None);
        assert_eq!(game.en_passant_pos, None);
        assert_eq!(game.castle_availability, [true, true, true, true]);
        assert_eq!(
            game.get_fen_notation(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_unmake_en_passant() {
        let mut game = Game::new_starting_game();

        game.move_piece(&ChessMove::from_xboard_algebraic_notation("e2e4"));
        game.move_piece(&ChessMove::from_xboard_algebraic_notation("b8c6"));
        game.move_piece(&ChessMove::from_xboard_algebraic_notation("e4e5"));
        game.move_piece(&ChessMove::from_xboard_algebraic_notation("d7d5"));
        game.move_piece(&ChessMove::from_xboard_algebraic_notation("e5e6"));

        game.unmove_move();
        game.unmove_move();

        assert_eq!(
            game.get_fen_notation(),
            "r1bqkbnr/pppppppp/2n5/4P3/8/8/PPPP1PPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_pawn_take_unmake() {
        let mut game = Game::new_starting_game();

        game.move_piece(&ChessMove::from_xboard_algebraic_notation("a2a4"));
        game.move_piece(&ChessMove::from_xboard_algebraic_notation("b7b5"));
        game.move_piece(&ChessMove::from_xboard_algebraic_notation("a4b5"));
        game.move_piece(&ChessMove::from_xboard_algebraic_notation("a7a5"));

        game.unmove_move();
        game.unmove_move();
        game.unmove_move();
        game.unmove_move();

        assert_eq!(
            game.get_fen_notation(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }
}
