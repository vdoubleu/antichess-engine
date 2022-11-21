use pleco::BitMove;
use std::collections::HashMap;

pub struct OpeningBook {
    openings: HashMap<String, BitMove>,
}
impl OpeningBook {
    pub fn new() -> OpeningBook {
        let openings: HashMap<String, BitMove> = HashMap::from([
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 1 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
                BitMove::new(1478),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq - 0 1"),
                BitMove::new(3070),
            ),
        ]);
        OpeningBook { openings }
    }
    pub fn get_move(&self, fen: &str) -> Option<BitMove> {
        self.openings.get(fen).cloned()
    }
}
impl Default for OpeningBook {
    fn default() -> Self {
        Self::new()
    }
}
