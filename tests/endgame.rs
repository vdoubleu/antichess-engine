use antichess_engine::engine::Engine;

use pleco::{board::FenBuildError, Board};

#[test]
fn test_endgame() -> Result<(), FenBuildError> {
    let mut game = Board::from_fen("2bqkbnr/3p2pp/2n2p2/4p3/8/1PP1P3/3PQPPP/2r1KBNR w - - 0 1")?;
    let mut engine = Engine::new();

    let m = engine.generate_move(&game);
    assert!(m.is_ok());

    let m = m.unwrap();

    game.apply_move(m);

    Ok(())
}

#[test]
fn test_endgame_forcetake() -> Result<(), FenBuildError> {
    let mut game = Board::from_fen("2bqk2r/1p1p1ppp/2pNp3/7P/8/8/R2KP3/2B2BnR b - - 0 1")?;
    let mut engine = Engine::new();

    let m = engine.generate_move(&game);
    assert!(m.is_ok());

    let m = m.unwrap();

    game.apply_move(m);

    Ok(())
}

#[test]
fn test_checkmate() -> Result<(), FenBuildError> {
    let mut game = Board::from_fen("6k1/4QNpp/2p5/7P/8/8/3KP3/2B3BR w - - 0 1")?;
    let mut engine = Engine::new();
    engine.params.debug_print = 2;

    let m = engine.generate_move(&game);
    assert!(m.is_ok());

    let m = m.unwrap();

    game.apply_move(m);

    assert!(game.checkmate());

    Ok(())
}
