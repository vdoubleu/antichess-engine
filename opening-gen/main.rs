use antichess_engine::chess_game::{ChessMove, Game};
use antichess_engine::engine::Engine;
use std::collections::HashMap;

use std::time::Duration;

use std::fs;

/// will create an opening book
fn main() -> std::io::Result<()> {
    let mut opening_book: HashMap<String, ChessMove> = HashMap::new();

    generate_opening_book(&mut opening_book);

    // for the crate name, I suggest keeping "crate" here if you are not sure, but
    // depending on where this opening code is getting called from, you may want to
    // instead replace it with "antichess_engine"
    write_opening_book_to_file(&opening_book, "opening.rs", "crate")?;

    Ok(())
}

fn generate_opening_book(store_main: &mut HashMap<String, ChessMove>) {
    fn gen_rec(
        engine: &mut Engine,
        store: &mut HashMap<String, ChessMove>,
        game: &mut Game,
        depth: u8,
    ) {
        if depth == 0 {
            return;
        }

        let best_move_option = engine.generate_move(game, game.player_turn);

        if let Ok(best_move) = best_move_option {
            store.insert(game.get_fen_notation(), best_move);
        } else {
            return;
        }

        let moves = game.all_valid_moves_for_color(game.player_turn);

        for m in moves {
            game.move_piece(&m);

            gen_rec(engine, store, game, depth - 1);

            game.unmove_move();
        }
    }

    let recursion_depth = 4;
    let reasonable_search_depth = 6;
    let timeout = 60; // in seconds

    let mut game = Game::new_starting_game();
    let mut engine = Engine::new();

    engine.params.depth = reasonable_search_depth;
    engine.params.max_time = Duration::from_secs(timeout);

    gen_rec(&mut engine, store_main, &mut game, recursion_depth);
}

fn write_opening_book_to_file(
    opening_book: &HashMap<String, ChessMove>,
    file_name: &str,
    crate_name: &str,
) -> std::io::Result<()> {
    let mut content = String::new();

    let header = String::from("use ")
        + crate_name
        + "::chess_game::ChessMove; 
use std::collections::HashMap; 

pub struct OpeningBook { 
    openings: HashMap<String, ChessMove>, 
} 
impl OpeningBook { 
    pub fn new() -> OpeningBook { 
        let openings: HashMap<String, ChessMove> = HashMap::from([
";

    let footer = "
        ]);
        OpeningBook { openings, } 
    } 
    pub fn get_move(&self, fen: &str) -> Option<ChessMove> { 
        self.openings.get(fen).cloned() 
    } 
}
impl Default for OpeningBook {
    fn default() -> Self {
        Self::new()
    }
}";

    content.push_str(header.as_str());

    for (fen, chess_move) in opening_book {
        let chess_move_str = if chess_move.promotion.is_none() {
            format!(
                "ChessMove::new({}, {}, None)",
                chess_move.start_pos, chess_move.end_pos
            )
        } else {
            format!(
                "ChessMove::new({}, {}, Some({:?}))",
                chess_move.start_pos,
                chess_move.end_pos,
                chess_move.promotion.unwrap()
            )
        };

        let line = format!("(String::from(\"{}\"), {}),\n", fen, chess_move_str);
        content.push_str(line.as_str());
    }

    content.push_str(footer);

    fs::write(file_name, content)?;

    Ok(())
}
