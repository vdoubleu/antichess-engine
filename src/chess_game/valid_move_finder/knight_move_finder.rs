use crate::chess_game::{Color, Game, Pos};

use crate::chess_game::pos::PosExt;

pub fn all_valid_moves_for_knight(game: &Game, pos: Pos, color: Color) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let moves: Vec<i8> = vec![12, 21, 19, 8, -12, -21, -19, -8];

    for move_dist in moves {
        let new_pos = ((pos as i8) + move_dist) as usize;

        if !new_pos.on_board() {
            continue;
        }

        if game.board[new_pos].is_none() || game.has_piece_with_color(new_pos, color.opposite()) {
            valid_moves.push(new_pos);
        }
    }

    valid_moves
}

#[cfg(test)]
mod knight_tests {
    use super::*;

    #[test]
    fn test_valid_moves() {
        let game = Game::from_fen_notation("8/8/8/8/3N4/8/8/8");
        let knight_pos = Pos::from_alg_notation("d4");

        let valid_moves = all_valid_moves_for_knight(&game, knight_pos, Color::White);

        assert_eq!(valid_moves.len(), 8);

        let expected_moves = vec![
            Pos::from_alg_notation("c6"),
            Pos::from_alg_notation("e6"),
            Pos::from_alg_notation("f5"),
            Pos::from_alg_notation("f3"),
            Pos::from_alg_notation("e2"),
            Pos::from_alg_notation("c2"),
            Pos::from_alg_notation("b3"),
            Pos::from_alg_notation("b5"),
        ];

        for expected_move in expected_moves {
            assert!(valid_moves.contains(&expected_move));
        }
    }
}
