mod chess_game;
mod engine;
mod error;

use crate::chess_game::{ChessMove, Color, Game};
use crate::engine::{opening::OpeningBook, Engine};

use anyhow::{Context, Result};

use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the color of the engine_color
    #[clap(value_enum, default_value = "white")]
    color: Color,

    /// Debug level, -1, 0, 1, or 2. -1 is zero debug output, this is for actually playing games. 0
    /// is for no search progress output, 1 is for basic search progress output, 2 is for verbose
    #[clap(short, long, default_value = "1")]
    debug: i8,
}

fn print_move_list(moves: &Vec<ChessMove>) {
    eprintln!("valid moves: ");
    for m in moves {
        eprint!("{} ", m);
    }
    eprintln!();
}

fn generate_with_fallback(ab_engine: &mut Engine, game: &Game, color: Color) -> Result<ChessMove> {
    match ab_engine.generate_move(game, color) {
        Ok(m) => Ok(m),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Falling back to random move");
            ab_engine.generate_rand_move(game, color)
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut game = Game::new_starting_game();

    let stdin = io::stdin();

    let your_color = args.color;
    let opp_color = args.color.opposite();

    let mut engine = Engine::new();
    engine.opening_book = Some(OpeningBook::new());
    engine.params.debug_print = args.debug;

    if your_color == Color::White {
        let m = match generate_with_fallback(&mut engine, &game, Color::White) {
            Ok(m) => {
                println!("{}", m);
                m
            }
            Err(e) => {
                if args.debug > -1 {
                    eprintln!("encountered error while generating move: {}", e);
                    println!("No moves available");
                }
                return;
            }
        };
        if game.move_piece(&m).is_err() {
            if args.debug > -1 {
                eprintln!("invalid move generated: {}", m);
            }
            return;
        }

        if let Some(winner) = game.winner {
            if args.debug > -1 {
                println!("Game over. Winner: {}", winner);
            }
            return;
        }
    }

    eprintln!("{}", game);

    if args.debug > -1 {
        let opp_valid_moves = game.all_valid_moves_for_color(opp_color);
        print_move_list(&opp_valid_moves);
    }

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                // we can just error if we can't parse move because we assume the opponent always
                // returns valid moves. If they don't, we'll just error out.
                let opponent_move = ChessMove::from_xboard_algebraic_notation(&line)
                    .context("opponent inputted invalid move in main")
                    .unwrap();
                if game.move_piece(&opponent_move).is_err() {
                    if args.debug > -1 {
                        eprintln!("invalid move supplied: {}", opponent_move);
                    }
                    return;
                }

                if let Some(winner) = game.winner {
                    if args.debug > -1 {
                        println!("Game over. Winner: {}", winner);
                        eprintln!("turns: {}", game.turn_counter);
                    }
                    return;
                }

                let m = match generate_with_fallback(&mut engine, &game, your_color) {
                    Ok(m) => {
                        println!("{}", m);
                        m
                    }
                    Err(e) => {
                        if args.debug > -1 {
                            eprintln!("encountered error in move gen: {}", e);
                            println!("resign");
                        }
                        return;
                    }
                };

                if game.move_piece(&m).is_err() {
                    if args.debug > -1 {
                        eprintln!("invalid move generated: {}", m);
                    }
                    return;
                }

                if let Some(winner) = game.winner {
                    if args.debug > -1 {
                        println!("Game over. Winner: {}", winner);
                        eprintln!("turns: {}", game.turn_counter);
                    }
                    return;
                }

                if args.debug > -1 {
                    eprintln!("{}", game);
                }

                if args.debug > -1 {
                    let opp_valid_moves = game.all_valid_moves_for_color(opp_color);
                    print_move_list(&opp_valid_moves);
                }
            }
            Err(error) => {
                if args.debug > -1 {
                    println!("error: {}", error)
                }
            }
        }
    }
}
