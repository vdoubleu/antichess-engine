mod chess_game;
mod engine;
mod error;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::{opening::OpeningBook, Engine};

use anyhow::Context;

use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the color of the engine_color
    #[clap(value_enum, default_value = "white")]
    color: Color,

    /// Debug level, -1, 0, 1, or 2
    #[clap(short, long, default_value = "0")]
    debug: i8,
}

fn print_move_list(moves: &Vec<ChessMove>) {
    eprintln!("valid moves: ");
    for m in moves {
        eprint!("{} ", m);
    }
    eprintln!();
}

fn main() {
    let args = Args::parse();

    let mut game = Game::new_starting_game();

    let stdin = io::stdin();

    let your_color = args.color;
    let opp_color = args.color.opposite();

    let mut engine = Engine::new();
    engine.opening_book = Some(OpeningBook::new());

    if your_color == Color::White {
        let m = match engine.generate_move(&game, Color::White) {
            Ok(m) => {
                println!("{}", m);
                m
            }
            Err(e) => {
                eprintln!("encountered error while generating move: {}", e);
                println!("No moves available");
                return;
            }
        };
        game.move_piece(&m);

        if let Some(winner) = game.winner {
            println!("Game over. Winner: {}", winner);
            return;
        }
    }

    eprintln!("{}", game);

    let opp_valid_moves = game.all_valid_moves_for_color(opp_color);
    print_move_list(&opp_valid_moves);

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                // we can just error if we can't parse move because we assume the opponent always
                // returns valid moves. If they don't, we'll just error out.
                let opponent_move = ChessMove::from_xboard_algebraic_notation(&line)
                    .context("opponent inputted invalid move in main")
                    .unwrap();
                game.move_piece(&opponent_move);

                if let Some(winner) = game.winner {
                    println!("Game over. Winner: {}", winner);
                    eprintln!("turns: {}", game.turn_counter);
                    return;
                }

                let m = match engine.generate_move(&game, your_color) {
                    Ok(m) => {
                        println!("{}", m);
                        m
                    }
                    Err(e) => {
                        eprintln!("encountered error in move gen: {}", e);
                        println!("resign");
                        return;
                    }
                };

                game.move_piece(&m);

                if let Some(winner) = game.winner {
                    println!("Game over. Winner: {}", winner);
                    eprintln!("turns: {}", game.turn_counter);
                    return;
                }

                eprintln!("{}", game);

                let opp_valid_moves = game.all_valid_moves_for_color(opp_color);
                print_move_list(&opp_valid_moves);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
