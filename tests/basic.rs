use pleco::{Board, Player};

use anyhow::{bail, Result};

#[test]
fn test_create_game() {
    let game = Board::start_pos();

    assert_eq!(game.turn(), Player::White);
    assert_eq!(
        game.fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
}

#[test]
fn test_move_piece() -> Result<()> {
    let mut game = Board::start_pos();

    // white moves
    game.apply_uci_move("e2e4");

    assert_eq!(game.turn(), Player::Black);
    assert_eq!(
        game.fen(),
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
    );

    // black moves
    game.apply_uci_move("e7e5");

    assert_eq!(game.turn(), Player::White);
    assert_eq!(
        game.fen(),
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"
    );

    // white moves
    game.apply_uci_move("f1c4");

    assert_eq!(game.turn(), Player::Black);
    assert_eq!(
        game.fen(),
        "rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 1 2"
    );

    Ok(())
}

#[test]
fn test_castle() -> Result<()> {
    // castle kingside
    let mut game = match Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 1 2") {
        Ok(game) => game,
        Err(e) => bail!("cannot create game from fen {:?}", e),
    };

    game.apply_uci_move("e1g1");

    assert_eq!(game.turn(), Player::Black);
    assert_eq!(
        game.fen(),
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 b kq - 2 2"
    );

    game.apply_uci_move("e8g8");

    assert_eq!(game.turn(), Player::White);
    assert_eq!(
        game.fen(),
        "r4rk1/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 w - - 3 3"
    );

    // castle queenside
    let mut game = match Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 1 2") {
        Ok(game) => game,
        Err(e) => bail!("cannot create game from fen {:?}", e),
    };

    game.apply_uci_move("e1c1");

    assert_eq!(game.turn(), Player::Black);
    assert_eq!(
        game.fen(),
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R b kq - 2 2"
    );

    game.apply_uci_move("e8c8");

    assert_eq!(game.turn(), Player::White);
    assert_eq!(
        game.fen(),
        "2kr3r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R w - - 3 3"
    );

    Ok(())
}

#[test]
fn test_pawn_promotion() -> Result<()> {
    let mut game = match Board::from_fen("k7/7P/8/8/8/8/8/K7 w - - 6 4") {
        Ok(game) => game,
        Err(e) => bail!("cannot create game from fen {:?}", e),
    };

    game.apply_uci_move("h7h8q");

    assert_eq!(game.turn(), Player::Black);
    assert_eq!(game.fen(), "k6Q/8/8/8/8/8/8/K7 b - - 0 4");

    Ok(())
}

#[test]
fn test_pawn_promotion_while_take() -> Result<()> {
    let mut game = match Board::from_fen("k5n1/7P/8/8/8/8/8/K7 w - - 0 4") {
        Ok(game) => game,
        Err(e) => bail!("cannot create game from fen {:?}", e),
    };

    game.apply_uci_move("h7g8r");

    assert_eq!(game.turn(), Player::Black);
    assert_eq!(game.fen(), "k5R1/8/8/8/8/8/8/K7 b - - 0 4");

    Ok(())
}
