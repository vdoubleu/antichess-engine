use crate::chess_game::Pos;

pub trait PosExt {
    fn new(row: usize, col: usize) -> Self;
    fn on_board(self) -> bool;
    fn from_row_col(row: usize, col: usize) -> Self;
    fn to_row_col(self) -> (usize, usize);
    fn row(self) -> usize;
    fn col(self) -> usize;
    fn to_row_col_as_i8(self) -> (i8, i8);
    fn from_alg_notation(alg_notation: &str) -> Self;
    fn to_alg_notation(self) -> String;
}

pub static BOARD_START: usize = 21;
pub static BOARD_END: usize = 98;

impl PosExt for Pos {
    fn new(row: usize, col: usize) -> Self {
        row * 10 + col + BOARD_START
    }

    fn on_board(self) -> bool {
        self >= BOARD_START && self <= BOARD_END && self % 10 != 0 && self % 10 != 9
    }

    fn from_row_col(row: usize, col: usize) -> Self {
        row * 10 + col + BOARD_START
    }

    fn to_row_col(self) -> (usize, usize) {
        let row = (self - BOARD_START) / 10;
        let col = (self - BOARD_START) % 10;
        (row, col)
    }

    fn row(self) -> usize {
        (self - BOARD_START) / 10
    }

    fn col(self) -> usize {
        (self - BOARD_START) % 10
    }

    fn to_row_col_as_i8(self) -> (i8, i8) {
        let (row, col) = self.to_row_col();
        (row as i8, col as i8)
    }

    fn from_alg_notation(alg_notation: &str) -> Self {
        let mut chars = alg_notation.chars();
        let col = chars.next().unwrap() as usize - 'a' as usize;
        let row = 8 - chars.next().unwrap().to_digit(10).unwrap() as usize;
        Self::from_row_col(row, col)
    }

    fn to_alg_notation(self) -> String {
        let (row, col) = self.to_row_col();
        let col = (col + ('a' as usize)) as u8 as char;
        let row: char = char::from_digit(7 - (row as u32) + 1, 10).unwrap();
        format!("{}{}", col, row)
    }
}

#[cfg(test)]
mod pos_tests {
    use super::*;

    #[test]
    fn test_is_on_board() {
        let test_pos: Vec<(Pos, bool)> = vec![
            (0, false),
            (20, false),
            (21, true),
            (28, true),
            (29, false),
            (30, false),
            (99, false),
            (100, false),
            (101, false),
        ];

        for (pos, expected) in test_pos {
            assert_eq!(pos.on_board(), expected);
        }
    }

    #[test]
    fn test_to_from_row_col() {
        let test_pos: Vec<((usize, usize), Pos)> = vec![
            ((0, 0), 21),
            ((0, 1), 22),
            ((0, 7), 28),
            ((1, 0), 31),
            ((1, 1), 32),
            ((1, 7), 38),
            ((7, 0), 91),
            ((7, 1), 92),
            ((7, 7), 98),
        ];

        for ((row, col), expected) in test_pos {
            assert_eq!(Pos::from_row_col(row, col), expected);
            assert_eq!(expected.to_row_col(), (row, col));
        }
    }

    #[test]
    fn test_to_from_alg_notation() {
        let test_pos: Vec<(&str, Pos)> = vec![
            ("a1", 91),
            ("a2", 81),
            ("a8", 21),
            ("h1", 98),
            ("h2", 88),
            ("h8", 28),
        ];

        for (alg_notation, expected) in test_pos {
            assert_eq!(Pos::from_alg_notation(alg_notation), expected);
            assert_eq!(expected.to_alg_notation(), alg_notation);
        }
    }
}
