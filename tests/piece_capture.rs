use antichess_engine::chess_game::{ChessMove, Game};

#[test]
fn test_pawn_capture() {
    let mut game = Game::from_fen_notation("8/8/8/4p1p1/5p2/4P1P1/8/8");

    let m = ChessMove::from_xboard_algebraic_notation("g3f4");

    game.move_piece(&m);

    assert_eq!(game.get_fen_notation(), "8/8/8/4p1p1/5P2/4P3/8/8",);

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("g5f4"));

    assert_eq!(game.get_fen_notation(), "8/8/8/4p3/5p2/4P3/8/8",);

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e3f4"));

    assert_eq!(game.get_fen_notation(), "8/8/8/4p3/5P2/8/8/8",);

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e5f4"));

    assert_eq!(game.get_fen_notation(), "8/8/8/8/5p2/8/8/8",);
}

#[test]
fn test_en_passant() {
    let mut game = Game::from_fen_notation("8/8/8/8/p7/8/1P6/8");

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("b2b4"));

    assert_eq!(game.get_fen_notation(), "8/8/8/8/pP6/8/8/8",);

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("a4b3"));

    assert_eq!(game.get_fen_notation(), "8/8/8/8/8/1p6/8/8",);
}
