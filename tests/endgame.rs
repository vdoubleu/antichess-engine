use antichess_engine::chess_game::{Color, Game};
use antichess_engine::engine::generate_move;

#[test]
fn test_endgame() {
    let mut game = Game::from_fen_notation("2bqkbnr/3p2pp/2n2p2/4p3/8/1PP1P3/3PQPPP/2r1KBNR");

    let m = generate_move(&game, Color::White);
    assert!(m.is_some());

    let m = m.unwrap();

    game.move_piece(&m);
}
