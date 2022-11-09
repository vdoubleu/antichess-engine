mod chess_game;
mod engine;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::{generate_move, opening::OpeningBook, store::AlphaBetaStore};

use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the color of the engine_color
    #[clap(value_enum, default_value = "white")]
    color: Color,

    /// Debug level, 0, 1, or 2
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

    let mut store = AlphaBetaStore::new();

    let opening_book = OpeningBook::new();

    if your_color == Color::White {
        let m = match generate_move(&game, &mut store, Color::White, Some(&opening_book)) {
            Some(m) => {
                println!("{}", m);
                m
            }
            None => {
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
                let opponent_move = ChessMove::from_xboard_algebraic_notation(&line);
                game.move_piece(&opponent_move);

                if let Some(winner) = game.winner {
                    println!("Game over. Winner: {}", winner);
                    eprintln!("turns: {}", game.turn_counter);
                    return;
                }

                let m = match generate_move(&game, &mut store, your_color, Some(&opening_book)) {
                    Some(m) => {
                        println!("{}", m);
                        m
                    }
                    None => {
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
