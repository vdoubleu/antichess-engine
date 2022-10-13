use crate::game::{Square, Pos};

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