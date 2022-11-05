use crate::chess_game::{ChessMove, Color, Game};
use rand::prelude::SliceRandom;

#[allow(dead_code)]
pub fn random_move(game: &Game, color: Color) -> Option<ChessMove> {
    let all_valid_moves = game.all_valid_moves_for_color(color);
    let mut rng = rand::thread_rng();
    all_valid_moves.choose(&mut rng).copied()
}

#[cfg(test)]
mod random_move_tests {
    use super::*;
    use crate::chess_game::Game;

    #[test]
    fn test_random_move() {
        let mut game = Game::new_starting_game();
        let move_1 = random_move(&game, Color::White);
        assert!(move_1.is_some());

        game.move_piece(&move_1.unwrap());

        let move_2 = random_move(&game, Color::Black);
        assert!(move_2.is_some());

        game.move_piece(&move_2.unwrap());
    }
}
