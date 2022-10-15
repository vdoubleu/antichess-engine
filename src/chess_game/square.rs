use crate::chess_game::{Pos, Square};

impl Square {
    pub fn new() -> Square {
        Square {
            pos: Pos::default(),
            piece: None,
        }
    }

    pub fn char_notation(&self) -> String {
        match self.piece {
            Some(p) => p.char_notation(),
            None => String::from(" "),
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Square::new()
    }
}
