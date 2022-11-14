use crate::chess_game::ChessMove;
use std::collections::HashMap;

pub struct OpeningBook {
    openings: HashMap<String, ChessMove>,
}
impl OpeningBook {
    pub fn new() -> OpeningBook {
        let openings: HashMap<String, ChessMove> = HashMap::from([
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(73, 52, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(86, 66, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(76, 55, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(72, 62, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(65, 55, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(67, 57, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(83, 63, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(87, 77, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(85, 75, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(76, 66, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(62, 53, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(71, 52, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(93, 57, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(81, 61, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(68, 58, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(95, 84, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(81, 61, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(83, 63, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(96, 41, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(68, 58, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b"),
                ChessMove::new(32, 52, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(96, 41, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(73, 52, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(78, 97, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(73, 63, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(77, 67, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(71, 63, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(73, 52, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(93, 48, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(76, 97, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(71, 52, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(61, 51, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(81, 61, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(96, 41, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(95, 85, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(87, 77, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b"),
                ChessMove::new(38, 58, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(67, 57, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(87, 77, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(93, 57, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(66, 56, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(78, 57, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(78, 97, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(93, 48, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(67, 58, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(72, 62, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(72, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(96, 63, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(87, 77, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(96, 52, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(68, 57, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(93, 57, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(83, 63, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(96, 52, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(71, 52, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(76, 57, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(67, 56, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b"),
                ChessMove::new(38, 58, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(86, 66, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(71, 52, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(83, 63, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(75, 65, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(94, 58, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(66, 57, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(95, 84, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(86, 66, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b"),
                ChessMove::new(33, 53, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(83, 63, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(95, 85, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(98, 88, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(63, 54, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(67, 57, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(77, 67, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(83, 63, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(73, 54, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(64, 55, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(96, 41, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(62, 51, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(86, 66, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(81, 71, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(82, 72, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(76, 55, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(81, 61, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(78, 57, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(94, 58, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(66, 55, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(81, 61, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(83, 63, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(71, 52, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(93, 48, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(65, 56, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b"),
                ChessMove::new(35, 55, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(98, 97, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(87, 77, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(93, 57, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(93, 48, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(65, 54, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(71, 52, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(88, 78, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(87, 77, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b"),
                ChessMove::new(27, 46, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(81, 61, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(63, 52, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(73, 52, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(81, 71, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(95, 86, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/8/p7/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(95, 84, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(94, 58, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(86, 66, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(78, 57, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/5P2/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b"),
                ChessMove::new(38, 48, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(95, 84, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/3p4/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(66, 56, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(76, 97, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(64, 53, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b"),
                ChessMove::new(22, 43, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(73, 52, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/1P6/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/ppp1pppp/8/3p4/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(85, 65, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b"),
                ChessMove::new(27, 48, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/8/P7/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/2P5/8/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/3P4/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(92, 84, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkbnr/1ppppppp/p7/8/5P2/8/PPPPP1PP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(61, 52, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(71, 63, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(88, 68, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/6P1/8/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(82, 62, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/7n/8/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/6p1/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/8/7N/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(86, 66, None),
            ),
            (
                String::from("rnbqkbnr/pppppp1p/8/6p1/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(93, 82, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b"),
                ChessMove::new(22, 41, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/8/4p3/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(78, 68, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/5p2/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pppp1ppp/4p3/8/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(78, 68, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/8/7P/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/8/7p/3P4/8/PPP1PPPP/RNBQKBNR w"),
                ChessMove::new(87, 67, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/1p6/8/8/N7/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(83, 73, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/1P6/8/P1PPPPPP/RNBQKBNR w"),
                ChessMove::new(86, 76, None),
            ),
            (
                String::from("rnbqkbnr/ppppp1pp/8/5p2/8/5N2/PPPPPPPP/RNBQKB1R w"),
                ChessMove::new(92, 73, None),
            ),
            (
                String::from("rnbqkbnr/ppppppp1/7p/8/8/4P3/PPPP1PPP/RNBQKBNR w"),
                ChessMove::new(97, 85, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/2n5/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/P7/8/1PPPPPPP/RNBQKBNR w"),
                ChessMove::new(61, 51, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/2p5/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkbnr/pp1ppppp/8/2p5/8/2N5/PPPPPPPP/R1BQKBNR w"),
                ChessMove::new(81, 61, None),
            ),
            (
                String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(97, 78, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(86, 66, None),
            ),
            (
                String::from("rnbqkbnr/p1pppppp/8/1p6/7P/8/PPPPPPP1/RNBQKBNR w"),
                ChessMove::new(97, 76, None),
            ),
            (
                String::from("rnbqkb1r/pppppppp/5n2/8/8/2P5/PP1PPPPP/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
            ),
            (
                String::from("r1bqkbnr/pppppppp/n7/8/8/6P1/PPPPPP1P/RNBQKBNR w"),
                ChessMove::new(92, 71, None),
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
