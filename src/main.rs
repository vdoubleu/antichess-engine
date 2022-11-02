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

fn print_move_list(moves: &Vec<ChessMove>) {
    println!("valid moves: ");
    for m in moves {
        print!("{} ", m);
    }
    println!();
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
        game.make_move(&m);

        if let Some(winner) = game.game_winner() {
            println!("Game over. Winner: {}", winner);
            return;
        }

        println!("{}", game);

        let opp_valid_moves = game.all_valid_moves_for_color(args.color.opposite());
        print_move_list(&opp_valid_moves);
    }

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let opponent_move = ChessMove::from_xboard_algebraic_notation(&line, &game);
                game.make_move(&opponent_move);

                if let Some(winner) = game.game_winner() {
                    println!("Game over. Winner: {}", winner);
                    return;
                }

                let m = match generate_move(&game, Color::White) {
                    Some(m) => m,
                    None => {
                        println!("resign");
                        return;
                    }
                };

                game.make_move(&m);

                if let Some(winner) = game.game_winner() {
                    println!("Game over. Winner: {}", winner);
                    return;
                }

                println!("{}", game);

                let opp_valid_moves = game.all_valid_moves_for_color(args.color.opposite());
                print_move_list(&opp_valid_moves);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
