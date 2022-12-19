mod engine;
mod error;

use crate::engine::{opening::OpeningBook, Engine};

use anyhow::Result;

use pleco::{BitMove, Board, Player};

use clap::{App, Arg};
use std::io::{self, BufRead};

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
            if ab_engine.params.debug_print > -1 {
                eprintln!("Error: {}", e);
                eprintln!("Falling back to random move");
            }
            ab_engine.generate_rand_move(board)
        }
    }
}

fn print_winner(player: Player, is_draw: bool) {
    if is_draw {
        println!("1/2-1/2");
        return;
    }

    match player {
        Player::White => println!("1-0"),
        Player::Black => println!("0-1"),
    }
}

fn main() {
    // we are now using clap 3, so we have to downgrade from clap 4
    let matches = App::new("Anti-fish")
        .version("0.1.0")
        .author("Victor Wang, Patrick Wang, Jae Yoon Cha")
        .about("The Anti-fish engine")
        .arg(
            Arg::with_name("color")
                .value_name("COLOR")
                .help("the color of the engine")
                .takes_value(true)
                .possible_values(&["black", "white"])
                .default_value("white"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .value_name("DEBUG")
                .help("Debug level, -1, 0, 1, or 2. -1 is zero debug output, this is for actually playing games. 0 is for no search progress output, 1 is for basic search progress output, 2 is for verbose")
                .takes_value(true)
                .default_value("-1"),
        )
        .get_matches();

    let your_color = match matches.value_of("color") {
        Some("black") => Player::Black,
        Some("white") => Player::White,
        _ => panic!("Invalid color"),
    };

    let debug_level = match matches.value_of("debug").unwrap().parse::<i8>() {
        Ok(n) => n,
        Err(_) => panic!("Invalid debug level"),
    };

    let mut board = Board::start_pos();

    let stdin = io::stdin();

    let mut engine = Engine::new();
    engine.opening_book = Some(OpeningBook::new());
    engine.params.debug_print = debug_level;

    if your_color == Player::White {
        let m = match generate_with_fallback(&mut engine, &board) {
            Ok(m) => {
                println!("{}", m);
                m
            }
            Err(e) => {
                if debug_level > -1 {
                    eprintln!("encountered error while generating move: {}", e);
                    println!("No moves available");
                }
                return;
            }
        };

        board.apply_move(m);
    }

    if debug_level > -1 {
        eprintln!("{}", board);

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
                    continue;
                }

                if board.checkmate() || board.stalemate() {
                    if debug_level > -1 {
                        println!("Game over. winner: {}", board.turn().other_player());
                        eprintln!("{}", board);
                        eprintln!("ply: {}", board.ply());
                        eprintln!("total time: {}", engine.store.total_search_time_ms);
                    }

                    print_winner(board.turn().other_player(), board.stalemate());

                    return;
                }

                let m = match generate_with_fallback(&mut engine, &board) {
                    Ok(m) => {
                        println!("{}", m);
                        m
                    }
                    Err(e) => {
                        if debug_level > -1 {
                            eprintln!("encountered error in move gen: {}", e);
                            eprintln!("resign");
                        }
                        return;
                    }
                };

                board.apply_move(m);

                if board.checkmate() || board.stalemate() {
                    if debug_level > -1 {
                        println!("Game over. winner: {}", board.turn().other_player());
                        eprintln!("{}", board);
                        eprintln!("ply: {}", board.ply());
                        eprintln!("total time: {}", engine.store.total_search_time_ms);
                    }

                    print_winner(board.turn().other_player(), board.stalemate());

                    return;
                }

                if debug_level > -1 {
                    eprintln!("{}", board);

                    let opp_valid_moves = engine.generate_valid_moves(&board);
                    print_move_list(&opp_valid_moves);
                }
            }
            Err(error) => {
                if debug_level > -1 {
                    println!("error: {}", error)
                }
            }
        }
    }
}
