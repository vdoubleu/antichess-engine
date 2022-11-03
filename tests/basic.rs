use antichess_engine::chess_game::{ChessMove, Color, Game};

#[test]
fn test_create_game() {
    let game = Game::new_starting_game();

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );
}

#[test]
fn test_move_piece() {
    let mut game = Game::new_starting_game();

    // white moves
    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e2e4"));

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR"
    );

    // black moves
    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e7e5"));

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR"
    );

    // white moves
    game.move_piece(&ChessMove::from_xboard_algebraic_notation("f1c4"));

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR"
    );
}

#[test]
fn test_castle() {
    // castle kingside
    let mut game = Game::from_fen_notation("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e1g1"));

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R4RK1"
    );

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e8g8"));

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "r4rk1/pppppppp/8/8/8/8/PPPPPPPP/R4RK1"
    );

    // castle queenside
    let mut game = Game::from_fen_notation("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e1c1"));

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R"
    );

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("e8c8"));

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "2kr3r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R"
    );
}

#[test]
fn test_pawn_promotion() {
    let mut game = Game::from_fen_notation("8/7P/8/8/8/8/8/8");

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("h7h8q"));

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(game.get_fen_notation(), "7Q/8/8/8/8/8/8/8");
}

#[test]
fn test_pawn_promotion_while_take() {
    let mut game = Game::from_fen_notation("6n1/7P/8/8/8/8/8/8");

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("h7g8r"));

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(game.get_fen_notation(), "6R1/8/8/8/8/8/8/8");
}
