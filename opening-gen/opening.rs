use crate::chess_game::ChessMove; 
use std::collections::HashMap; 

pub struct OpeningBook { 
    openings: HashMap<String, ChessMove>, 
} 
impl OpeningBook { 
    pub fn new() -> OpeningBook { 
        let openings: HashMap<String, ChessMove> = HashMap::from([
(String::from("rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b"), ChessMove::new(27, 46, None)),(String::from("rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b"), ChessMove::new(27, 46, None)),(String::from("rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b"), ChessMove::new(32, 52, None)),(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"), ChessMove::new(92, 71, None)),(String::from("rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b"), ChessMove::new(27, 46, None)),
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