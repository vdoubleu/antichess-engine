use anyhow::Result;

use pleco::{BitMove, Board};

use std::time::Instant;

use std::collections::{HashMap, HashSet};

pub struct TranspositionTableEntry {
    pub depth: i32,
    pub chess_move: Option<BitMove>,
    pub fen: String,
    pub score: f64,
    pub flag: TranspositionTableFlag,
    pub ply: u16,
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
    pub transposition_table: HashMap<u64, TranspositionTableEntry>,

    pub pv: Vec<BitMove>,

    pub total_search_time_ms: u128,
}

impl AlphaBetaStore {
    pub fn new() -> Self {
        AlphaBetaStore {
            start_time: Some(Instant::now()),
            curr_depth: 0,
            transposition_table: HashMap::new(),
            pv: Vec::new(),
            total_search_time_ms: 0,
        }
    }
    pub fn store_transposition(
        &mut self,
        board: &Board,
        depth: i32,
        score: f64,
        chess_move: Option<BitMove>,
        node_type: TranspositionTableFlag,
    ) {
        if let Some(existing_entry) = self.get_transposition(board) {
            if existing_entry.1 && existing_entry.0.depth >= depth {
                return;
            }
        }

        let zobrist = board.zobrist();
        let entry = TranspositionTableEntry {
            depth,
            chess_move,
            fen: board.fen(),
            score,
            flag: node_type,
            ply: board.ply(),
        };

        self.transposition_table.insert(zobrist, entry);
    }

    pub fn get_transposition(&self, board: &Board) -> Option<(&TranspositionTableEntry, bool)> {
        let hash = self.hash(board);
        let res = self.transposition_table.get(&hash);

        if let Some(entry) = res {
            if entry.fen == board.fen() {
                let is_current_ply = entry.ply == board.ply();
                Some((entry, is_current_ply))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn start_turn(&mut self) {
        self.transposition_table.clear();
        self.start_time = Some(Instant::now());
    }

    pub fn end_turn(&mut self) {
        self.start_time = None;
    }

    fn hash(&self, board: &Board) -> u64 {
        board.zobrist()
    }

    pub fn probe_fill_pv(&mut self, board: &mut Board) -> Result<()> {
        let mut transpo = self.get_transposition(board);

        let mut move_ind = 0;

        let mut seen: HashSet<String> = HashSet::new();

        while transpo.is_some() {
            let (transpo_entry, ply_is_current) = transpo.unwrap();

            if !ply_is_current {
                break;
            }

            if seen.contains(&transpo_entry.fen) {
                break;
            }

            if transpo_entry.chess_move.is_none() {
                break;
            }

            if board.checkmate() || board.stalemate() {
                break;
            }

            seen.insert(transpo_entry.fen.clone());

            let transpo_move = transpo_entry.chess_move.unwrap();
            if move_ind >= self.pv.len() {
                self.pv.push(transpo_move);
            } else {
                self.pv[move_ind] = transpo_move;
            }
            board.apply_move(transpo_move);

            move_ind += 1;

            transpo = self.get_transposition(board);
        }

        for _ in 0..move_ind {
            board.undo_move();
        }

        Ok(())
    }
}

impl Default for AlphaBetaStore {
    fn default() -> Self {
        AlphaBetaStore::new()
    }
}
