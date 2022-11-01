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

    /// Returns the algebraic notation of the position
    /// Pos { row: 0, col: 0 } -> "a1"
    pub fn get_algebraic_notation(&self) -> String {
        let mut notation = String::new();
        notation.push((self.col as u8 + 97) as char);
        notation.push(((7 - self.row) as u8 + 49) as char);
        notation
    }

    /// Takes a chess position in standard chess notation and returns a Pos
    /// For example, "a1" would return Pos { row: 7, col: 0 }
    pub fn from_algebraic_notation(s: String) -> Pos {
        let col = s.chars().next().unwrap() as usize - 97;
        let row = 7 - (s.chars().nth(1).unwrap() as usize - 49);
        Pos::new(row, col)
    }

    pub fn get_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Eq for Pos {}
