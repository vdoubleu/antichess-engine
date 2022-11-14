use antichess_engine::chess_game::{ChessMove, Color, Game};

use anyhow::Result;

#[test]
fn test_create_game() {
    let game = Game::new_starting_game();

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"
    );
}

#[test]
fn test_move_piece() -> Result<()> {
    let mut game = Game::new_starting_game();

    // white moves
    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e2e4")?)?;

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b"
    );

    // black moves
    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e7e5")?)?;

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w"
    );

    // white moves
    game.move_piece(&ChessMove::from_xboard_algebraic_notation("f1c4")?)?;

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR b"
    );

    Ok(())
}

#[test]
fn test_castle() -> Result<()> {
    // castle kingside
    let mut game = Game::from_fen_notation("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R")?;

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e1g1")?)?;

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 b"
    );

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e8g8")?)?;

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "r4rk1/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 w"
    );

    // castle queenside
    let mut game = Game::from_fen_notation("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R")?;

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e1c1")?)?;

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R b"
    );

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e8c8")?)?;

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "2kr3r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R w"
    );

    Ok(())
}

#[test]
fn test_pawn_promotion() -> Result<()> {
    let mut game = Game::from_fen_notation("8/7P/8/8/8/8/8/8")?;

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("h7h8q")?)?;

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(game.get_fen_notation(), "7Q/8/8/8/8/8/8/8 b");

    Ok(())
}

#[test]
fn test_pawn_promotion_while_take() -> Result<()> {
    let mut game = Game::from_fen_notation("6n1/7P/8/8/8/8/8/8")?;

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("h7g8r")?)?;

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(game.get_fen_notation(), "6R1/8/8/8/8/8/8/8 b");

    Ok(())
}
