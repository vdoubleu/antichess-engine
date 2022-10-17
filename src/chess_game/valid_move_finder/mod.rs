mod bishop_move_finder;
mod king_move_finder;
mod knight_move_finder;
mod pawn_move_finder;
mod queen_move_finder;
mod rook_move_finder;

use crate::chess_game::valid_move_finder::bishop_move_finder::all_valid_moves_for_bishop;
use crate::chess_game::valid_move_finder::king_move_finder::all_valid_moves_for_king;
use crate::chess_game::valid_move_finder::knight_move_finder::all_valid_moves_for_knight;
use crate::chess_game::valid_move_finder::pawn_move_finder::all_valid_moves_for_pawn;
use crate::chess_game::valid_move_finder::queen_move_finder::all_valid_moves_for_queen;
use crate::chess_game::valid_move_finder::rook_move_finder::all_valid_moves_for_rook;
use crate::chess_game::{Game, Pos};

fn move_within_board_bounds(row: i16, col: i16) -> bool {
    // assumes board is 8x8
    let r = 0..8;
    r.contains(&row) && r.contains(&col)
}

pub fn all_bishop_moves(game: &Game, pos: &Pos, no_piece_check: bool) -> Vec<Pos> {
    all_valid_moves_for_bishop(game, pos, no_piece_check)
}

pub fn all_king_moves(game: &Game, pos: &Pos, only_check_currently_attacking: bool) -> Vec<Pos> {
    all_valid_moves_for_king(game, pos, only_check_currently_attacking)
}

pub fn all_knight_moves(game: &Game, pos: &Pos) -> Vec<Pos> {
    all_valid_moves_for_knight(game, pos)
}

pub fn all_pawn_moves(game: &Game, pos: &Pos) -> Vec<Pos> {
    all_valid_moves_for_pawn(game, pos)
}

pub fn all_queen_moves(game: &Game, pos: &Pos) -> Vec<Pos> {
    all_valid_moves_for_queen(game, pos)
}

pub fn all_rook_moves(game: &Game, pos: &Pos, no_piece_check: bool) -> Vec<Pos> {
    all_valid_moves_for_rook(game, pos, no_piece_check)
}
