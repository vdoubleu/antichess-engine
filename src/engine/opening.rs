use crate::chess_game::ChessMove;
use std::collections::HashMap;

pub struct OpeningBook {
    openings: HashMap<String, ChessMove>,
}
impl OpeningBook {
    pub fn new() -> OpeningBook {
        let openings: HashMap<String, ChessMove> = HashMap::from([
            (
                String::from("rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b"),
                ChessMove::new(27, 48, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b"),
                ChessMove::new(38, 48, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b"),
                ChessMove::new(35, 55, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b"),
                ChessMove::new(33, 53, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b"),
                ChessMove::new(32, 52, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b"),
                ChessMove::new(37, 57, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b"),
                ChessMove::new(38, 58, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b"),
                ChessMove::new(33, 43, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b"),
                ChessMove::new(27, 46, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b"),
                ChessMove::new(27, 46, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b"),
                ChessMove::new(31, 41, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b"),
                ChessMove::new(27, 46, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b"),
                ChessMove::new(32, 52, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
        ]);
        OpeningBook { openings }
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
