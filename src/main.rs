mod chess_game;
mod engine;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::generate_move;

use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // the color of the engine_color
    #[clap(value_enum, default_value = "white")]
    color: Color,
}

fn main() {
    let args = Args::parse();

    let mut game = Game::new();
    game.set_starting_pos();

    let stdin = io::stdin();

    if args.color == Color::White {
        let m = match generate_move(&game, Color::White) {
            Some(m) => m,
            None => {
                println!("No moves available");
                return;
            }
        };
        game.make_move(&m, None);
    }

    println!("{}", game);

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let opponent_move = ChessMove::from_xboard_algebraic_notation(&line, &game);
                game.make_move(&opponent_move, None);

                let m = match generate_move(&game, Color::White) {
                    Some(m) => m,
                    None => {
                        println!("resign");
                        return;
                    }
                };

                game.make_move(&m, None);

                println!("{}", game);
            }
            Err(error) => println!("error: {}", error),
        }
    }

    //game.from_fen_notation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

    // println!("{}", game);

    // let moves = game.all_valid_moves_for_color(Color::White);
    // // println!("{:?}", moves);

    // // only look at last 5 moves
    // //let moves = moves.iter().rev().take(5).rev().collect::<Vec<_>>();

    // // for some reason, when printing all possible starting moves
    // // for white at once, the shell will chop off weird bits of the last
    // // board. I've double checked that everything is good, and it
    // // works fine if you print only parts of it at a time.
    // for m in moves {
    //     let mut game_copy = game.clone();
    //     game_copy.make_move(&m, None);

    //     println!("{}", m);
    //     println!("{}", game_copy);
    // }
}
