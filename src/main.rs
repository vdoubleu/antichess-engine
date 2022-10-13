mod game;

use crate::game::{Game, Color};

fn main() {
    let mut game = Game::new();
    game.set_starting_pos();
    //game.from_fen_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

    println!("{}", game);

    let moves = game.all_valid_moves_for_color(Color::White);
    println!("{:?}", moves);
}
