mod chess_game;

use crate::chess_game::{Color, Game};

fn main() {
    let mut game = Game::new();
    game.set_starting_pos();
    //game.from_fen_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

    println!("{}", game);

    let moves = game.all_valid_moves_for_color(Color::White);
    // println!("{:?}", moves);

    // only look at last 5 moves
    //let moves = moves.iter().rev().take(5).rev().collect::<Vec<_>>();

    // for some reason, when printing all possible starting moves
    // for white at once, the shell will chop off weird bits of the last
    // board. I've double checked that everything is good, and it
    // works fine if you print only parts of it at a time.
    for m in moves {
        let mut game_copy = game.clone();
        game_copy.make_move(&m, None);

        println!("{}", m);
        println!("{}", game_copy);
    }
}
