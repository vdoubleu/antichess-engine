// use antichess_engine::chess_game::{ChessMove, Game};
use antichess_engine::engine::Engine;

use anyhow::Result;

use pleco::{BitMove, Board};

use std::collections::HashMap;

use std::time::Duration;

use std::fs;

/// will create an opening book
fn main() -> Result<()> {
    let mut opening_book: HashMap<String, BitMove> = HashMap::new();

    generate_opening_book(&mut opening_book);

    write_opening_book_to_file(&opening_book, "opening.rs")?;

    Ok(())
}

fn generate_opening_book(store_main: &mut HashMap<String, BitMove>) {
    fn gen_rec(
        engine: &mut Engine,
        store: &mut HashMap<String, BitMove>,
        board: &mut Board,
        depth: u8,
    ) {
        if depth == 0 {
            return;
        }

        let best_move_option = engine.generate_move(board);

        if let Ok(best_move) = best_move_option {
            store.insert(board.fen(), best_move);
        } else {
            return;
        }

        let moves = board.generate_moves();

        for m in moves {
            board.apply_move(m);

            gen_rec(engine, store, board, depth - 1);

            board.undo_move();
        }
    }

    let recursion_depth = 2;
    let reasonable_search_depth = 8;
    let timeout = 60; // in seconds

    let mut game = Board::start_pos();
    let mut engine = Engine::new();

    engine.params.depth = reasonable_search_depth;
    engine.params.max_time = Duration::from_secs(timeout);

    gen_rec(&mut engine, store_main, &mut game, recursion_depth);
}

fn write_opening_book_to_file(
    opening_book: &HashMap<String, BitMove>,
    file_name: &str,
) -> Result<()> {
    let mut content = String::new();

    let header = "use pleco::BitMove;
use std::collections::HashMap; 

pub struct OpeningBook { 
    openings: HashMap<String, BitMove>, 
} 
impl OpeningBook { 
    pub fn new() -> OpeningBook { 
        let openings: HashMap<String, BitMove> = HashMap::from([
";

    let footer = "
        ]);
        OpeningBook { openings, } 
    } 
    pub fn get_move(&self, fen: &str) -> Option<BitMove> { 
        self.openings.get(fen).cloned() 
    } 
}
impl Default for OpeningBook {
    fn default() -> Self {
        Self::new()
    }
}";

    content.push_str(header);

    for (fen, chess_move) in opening_book {
        let chess_move_str = format!("BitMove::new({})", chess_move.get_raw());

        let line = format!("(String::from(\"{}\"), {}),\n", fen, chess_move_str);
        content.push_str(line.as_str());
    }

    content.push_str(footer);

    fs::write(file_name, content)?;

    Ok(())
}
