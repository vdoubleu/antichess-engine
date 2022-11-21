// use antichess_engine::chess_game::{ChessMove, Game};
use anyhow::Result;
use pleco::{board::FenBuildError, Board};

#[test]
fn test_pawn_capture() -> Result<(), FenBuildError> {
    let mut game = Board::from_fen("7k/8/8/4p1p1/5p2/4P1P1/8/7K w - - 0 1")?;

    game.apply_uci_move("g3f4");

    assert_eq!(game.fen(), "7k/8/8/4p1p1/5P2/4P3/8/7K b - - 0 1",);

    game.apply_uci_move("g5f4");

    assert_eq!(game.fen(), "7k/8/8/4p3/5p2/4P3/8/7K w - - 0 2",);

    game.apply_uci_move("e3f4");

    assert_eq!(game.fen(), "7k/8/8/4p3/5P2/8/8/7K b - - 0 2",);

    game.apply_uci_move("e5f4");

    assert_eq!(game.fen(), "7k/8/8/8/5p2/8/8/7K w - - 0 3",);

    Ok(())
}

#[test]
fn test_en_passant() -> Result<(), FenBuildError> {
    let mut game = Board::from_fen("7k/8/8/8/p7/8/1P6/7K w - - 0 3")?;

    game.apply_uci_move("b2b4");

    assert_eq!(game.fen(), "7k/8/8/8/pP6/8/8/7K b - b3 0 3",);

    game.apply_uci_move("a4b3");

    assert_eq!(game.fen(), "7k/8/8/8/8/1p6/8/7K w - - 0 4",);

    Ok(())
}
