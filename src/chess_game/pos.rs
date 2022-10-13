use crate::chess_game::Pos;

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

    pub fn to_algebraic_notation(&self) -> String {
        let mut notation = String::new();
        notation.push((self.col as u8 + 97) as char);
        notation.push(((7 - self.row) as u8 + 49) as char);
        notation
    }
}