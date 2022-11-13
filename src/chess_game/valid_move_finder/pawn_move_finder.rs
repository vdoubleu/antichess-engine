use crate::chess_game::pos::PosExt;
use crate::chess_game::{Color, Game, Pos};

pub fn all_valid_moves_for_pawn(
    game: &Game,
    pos: Pos,
    color: Color,
    only_check_currently_attacking: bool,
) -> Vec<Pos> {
    let mut valid_moves: Vec<Pos> = Vec::new();

    let pawn_moves: Vec<i8> = match color {
        Color::White => vec![-10, -20, -9, -11],
        Color::Black => vec![10, 20, 9, 11],
    };

    for move_dist in pawn_moves {
        let new_pos = (pos as i8 + move_dist) as Pos;

        if !new_pos.on_board() {
            continue;
        }

        // if moving diagonally, check if there is a piece to capture
        // either capture directly, or en passant
        if move_dist.abs() == 9 || move_dist.abs() == 11 {
            if game.has_piece_with_color(new_pos, color.opposite())
                || is_en_passant(game, pos, new_pos)
            {
                valid_moves.push(new_pos);
            }
            continue;
        }

        // this is used when trying to determine if you can castle. Moving forwards isn't attacking
        // and so pawns moving forward can't stop castling.
        if only_check_currently_attacking {
            continue;
        }

        // if moving forward, check if there is already a piece there
        if (move_dist.abs() == 10 || move_dist.abs() == 20) && game.board[new_pos].is_some() {
            continue;
        }

        // if wanting to move two squares forward
        if move_dist.abs() == 20 {
            // check if on starting row
            let starting_row = match color {
                Color::White => 6,
                Color::Black => 1,
            };

            if pos.row() != starting_row {
                // not on starting row, we cannot move two squares forward
                continue;
            }

            // check if there was a piece right in front blocking us
            let one_square_forward = (pos as i8 + move_dist / 2) as Pos;
            if game.board[one_square_forward].is_some() {
                continue;
            }
        }

        valid_moves.push(new_pos);
    }

    valid_moves
}

fn is_en_passant(game: &Game, pawn_curr_pos: Pos, move_pos: Pos) -> bool {
    if game.en_passant_pos.is_none() {
        return false;
    }

    let (ep_row, ep_col) = game.en_passant_pos.unwrap().to_row_col();

    if ep_row != pawn_curr_pos.row() {
        return false;
    }

    if ep_col != move_pos.col() {
        return false;
    }

    true
}

#[cfg(test)]
mod pawn_tests {
    use super::*;

    use crate::error::ChessError;

    #[test]
    fn test_valid_pawn_moves_black() -> Result<(), ChessError> {
        let game = Game::from_fen_notation("8/5p2/4P1P1/8/8/8/8/8")?;
        let pos = Pos::new(1, 5);

        let valid_moves = all_valid_moves_for_pawn(&game, pos, Color::Black, false);
        let expected_moves = vec![
            Pos::new(2, 5),
            Pos::new(3, 5),
            Pos::new(2, 4),
            Pos::new(2, 6),
        ];

        eprintln!("{:?}", valid_moves);
        assert!(valid_moves.iter().all(|m| expected_moves.contains(m)));

        Ok(())
    }

    #[test]
    fn test_valid_pawn_moves_white() -> Result<(), ChessError> {
        let game = Game::from_fen_notation("8/8/8/8/8/4p1p1/5P2/8")?;
        let pos = Pos::new(6, 5);

        let valid_moves = all_valid_moves_for_pawn(&game, pos, Color::White, false);
        let expected_moves = vec![
            Pos::new(5, 5),
            Pos::new(4, 5),
            Pos::new(5, 4),
            Pos::new(5, 6),
        ];

        eprintln!("{:?}", valid_moves);
        assert!(valid_moves.iter().all(|m| expected_moves.contains(m)));

        Ok(())
    }

    #[test]
    fn test_starting_double_move() -> Result<(), ChessError> {
        let game = Game::from_fen_notation("8/8/8/8/8/8/4P3/8")?;
        let pos = Pos::new(6, 4);

        let valid_moves = all_valid_moves_for_pawn(&game, pos, Color::White, false);
        let expected_moves = vec![Pos::new(5, 4), Pos::new(4, 4)];

        eprintln!("{:?}", valid_moves);
        assert_eq!(valid_moves.len(), 2);
        assert!(valid_moves.iter().all(|m| expected_moves.contains(m)));

        Ok(())
    }
}
