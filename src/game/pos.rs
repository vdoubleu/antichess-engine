use crate::game::Pos;

impl Pos {
    pub fn default() -> Pos {
        Pos { row: 0, col: 0 }
    }

    pub fn new(r: usize, c: usize) -> Pos {
        if r > 7 || c > 7 {
            panic!("Invalid position: ({}, {})", r, c);
        }

        Pos { row: r, col: c }
    }
}
