use antichess_engine::chess_game::{ChessMove, Game};
use antichess_engine::engine::Engine;

use anyhow::Result;

#[test]
fn test_pawn_capture() -> Result<()> {
    let mut game = Game::from_fen_notation("8/8/8/4p1p1/5p2/4P1P1/8/8")?;

    let m = ChessMove::from_xboard_algebraic_notation("g3f4")?;

    game.move_piece(&m)?;

    assert_eq!(game.get_fen_notation(), "8/8/8/4p1p1/5P2/4P3/8/8 b",);

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("g5f4")?)?;

    assert_eq!(game.get_fen_notation(), "8/8/8/4p3/5p2/4P3/8/8 w",);

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e3f4")?)?;

    assert_eq!(game.get_fen_notation(), "8/8/8/4p3/5P2/8/8/8 b",);

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e5f4")?)?;

    assert_eq!(game.get_fen_notation(), "8/8/8/8/5p2/8/8/8 w",);

    Ok(())
}

#[test]
fn test_en_passant() -> Result<()> {
    let mut game = Game::from_fen_notation("8/8/8/8/p7/8/1P6/8")?;

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("b2b4")?)?;

    assert_eq!(game.get_fen_notation(), "8/8/8/8/pP6/8/8/8 b",);

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("a4b3")?)?;

    assert_eq!(game.get_fen_notation(), "8/8/8/8/8/1p6/8/8 w",);

    Ok(())
}

#[test]
fn test_en_passant_2() -> Result<()> {
    let mut game = Game::from_fen_notation("1nbqkbnr/r1ppp1p1/5p1p/1p6/P4P1P/8/1PPPP1P1/1NBQKBNR")?;
    let mut engine = Engine::new();
    engine.params.depth = 6;

    let m = engine.generate_move(&game, game.player_turn);

    assert!(m.is_ok());

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("a4b5")?)?;

    let m = engine.generate_move(&game, game.player_turn);

    assert!(m.is_ok());

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("c7c5")?)?;

    let m = engine.generate_move(&game, game.player_turn);

    assert!(m.is_ok());

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("b5c6")?)?;

    Ok(())
}
