mod engine;
mod error;

use crate::engine::{opening::OpeningBook, Engine};

use anyhow::Result;

use pleco::{BitMove, Board, Player};

use clap::Parser;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorArg {
    Black = 0,
    White = 1,
}

impl ColorArg {
    pub fn to_player(self) -> Player {
        match self {
            ColorArg::Black => Player::Black,
            ColorArg::White => Player::White,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the color of the engine_color
    #[clap(value_enum, default_value = "white")]
    color: ColorArg,

    /// Debug level, -1, 0, 1, or 2. -1 is zero debug output, this is for actually playing games. 0
    /// is for no search progress output, 1 is for basic search progress output, 2 is for verbose
    #[clap(short, long, default_value = "1")]
    debug: i8,
}

fn print_move_list(moves: &Vec<BitMove>) {
    eprintln!("valid moves: ");
    for m in moves {
        eprint!("{} ", m);
    }
    eprintln!();
}

fn generate_with_fallback(ab_engine: &mut Engine, board: &Board) -> Result<BitMove> {
    match ab_engine.generate_move(board) {
        Ok(m) => Ok(m),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Falling back to random move");
            ab_engine.generate_rand_move(board)
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut board = Board::start_pos();

    let stdin = io::stdin();

    let your_color = args.color.to_player();

    let mut engine = Engine::new();
    engine.opening_book = Some(OpeningBook::new());
    engine.params.debug_print = args.debug;

    if your_color == Player::White {
        let m = match generate_with_fallback(&mut engine, &board) {
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

        board.apply_move(m);
    }

    eprintln!("{}", board);

    if args.debug > -1 {
        let opp_valid_moves = engine.generate_valid_moves(&board);
        print_move_list(&opp_valid_moves);
    }

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                // we can just error if we can't parse move because we assume the opponent always
                // returns valid moves. If they don't, we'll just error out.
                if !board.apply_uci_move(&line) {
                    eprintln!("Invalid move: {}", line);
                    return;
                }

                if board.checkmate() || board.stalemate() {
                    if args.debug > -1 {
                        eprintln!("Game over, winner: {}", board.turn().other_player());
                    }

                    return;
                }

                let m = match generate_with_fallback(&mut engine, &board) {
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

                board.apply_move(m);

                if board.checkmate() || board.stalemate() {
                    if args.debug > -1 {
                        eprintln!("Game over, winner: {}", board.turn().other_player());
                    }

                    return;
                }

                if args.debug > -1 {
                    eprintln!("{}", board);
                }

                if args.debug > -1 {
                    let opp_valid_moves = engine.generate_valid_moves(&board);
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
