use crate::chess_game::{ChessMove, Game};
use std::time::Instant;

use std::collections::{HashMap, HashSet};

pub struct TranspositionTableEntry {
    pub depth: i32,
    pub chess_move: Option<ChessMove>,
    pub fen: String,
    pub score: f64,
    pub flag: TranspositionTableFlag,
}

pub enum TranspositionTableFlag {
    Exact,
    Upper,
    Lower,
}

pub struct AlphaBetaStore {
    /// when the searching was started
    pub start_time: Option<Instant>,

    /// the current reasonable depth when searching
    /// we need to store this as well as the depth value inside
    /// the params because of iterative deepening
    pub curr_depth: i32,

    /// stores the transposition table
    pub transposition_table: HashMap<String, TranspositionTableEntry>,

    pub pv: Vec<ChessMove>,
}

impl AlphaBetaStore {
    pub fn new() -> Self {
        AlphaBetaStore {
            start_time: Some(Instant::now()),
            curr_depth: 0,
            transposition_table: HashMap::new(),
            pv: Vec::new(),
        }
    }
    pub fn store_transposition(
        &mut self,
        game: &Game,
        depth: i32,
        score: f64,
        chess_move: Option<ChessMove>,
        node_type: TranspositionTableFlag,
    ) {
        if let Some(existing_entry) = self.get_transposition(game) {
            if existing_entry.depth >= depth {
                return;
            }
        }

        let fen = game.get_fen_notation();
        let entry = TranspositionTableEntry {
            depth,
            chess_move,
            fen: fen.clone(),
            score,
            flag: node_type,
        };

        self.transposition_table.insert(fen, entry);
    }

    pub fn get_transposition(&self, game: &Game) -> Option<&TranspositionTableEntry> {
        let hash = self.hash(game);
        self.transposition_table.get(&hash)
    }

    pub fn start_turn(&mut self) {
        self.transposition_table.clear();
        self.start_time = Some(Instant::now());
    }

    pub fn end_turn(&mut self) {
        self.start_time = None;
    }

    fn hash(&self, game: &Game) -> String {
        game.get_fen_notation()
    }

    pub fn probe_fill_pv(&mut self, game: &mut Game) {
        let mut transpo = self.get_transposition(game);

        let mut move_ind = 0;

        let mut seen: HashSet<String> = HashSet::new();

        while transpo.is_some() {
            let transpo_entry = transpo.unwrap();

            if seen.contains(&transpo_entry.fen) {
                break;
            }

            if transpo_entry.chess_move.is_none() {
                break;
            }

            seen.insert(transpo_entry.fen.clone());

            let transpo_move = transpo_entry.chess_move.unwrap();
            if move_ind >= self.pv.len() {
                self.pv.push(transpo_move);
            } else {
                self.pv[move_ind] = transpo_move;
            }

            game.move_piece(&transpo_move);

            move_ind += 1;

            transpo = self.get_transposition(game);
        }

        for _ in 0..move_ind {
            game.unmove_move();
        }
    }
}

impl Default for AlphaBetaStore {
    fn default() -> Self {
        AlphaBetaStore::new()
    }
}
