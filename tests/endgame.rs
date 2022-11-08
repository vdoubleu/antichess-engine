use antichess_engine::chess_game::{ChessMove, Color, Game};
use antichess_engine::engine::generate_move;
use antichess_engine::engine::store::AlphaBetaStore;

#[test]
fn test_endgame() {
    let mut game = Game::from_fen_notation("2bqkbnr/3p2pp/2n2p2/4p3/8/1PP1P3/3PQPPP/2r1KBNR");
    let mut store = AlphaBetaStore::new();

    let m = generate_move(&game, &mut store, Color::White);
    assert!(m.is_some());

    let m = m.unwrap();

    game.move_piece(&m);
}

#[test]
fn test_endgame_forcetake() {
    let mut game = Game::from_fen_notation("2bqk2r/1p1p1ppp/2pNp3/7P/8/8/R2KP3/2B2BnR");
    let mut store = AlphaBetaStore::new();

    let m = generate_move(&game, &mut store, Color::White);
    assert!(m.is_some());

    let m = m.unwrap();

    assert_eq!(m, ChessMove::from_xboard_algebraic_notation("d6e8"));

    game.move_piece(&m);
}
