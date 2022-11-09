use crate::chess_game::ChessMove; 
use std::collections::HashMap; 

pub struct OpeningBook { 
    openings: HashMap<String, ChessMove>, 
} 
impl OpeningBook { 
    pub fn new() -> OpeningBook { 
        let openings: HashMap<String, ChessMove> = HashMap::from([
(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"), ChessMove::new(97, 78, None)),
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
}