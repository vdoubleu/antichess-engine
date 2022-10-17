use antichess_engine::chess_game::{ChessMove, Color, Game};

#[test]
fn test_create_game() {
    let mut game = Game::new();
    game.set_starting_pos();

    assert_eq!(game.get_player_turn(), Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );
}

#[test]
fn test_make_move() {
    let mut game = Game::new_starting_game();

    // white moves
    game.make_move(&ChessMove::from_xboard_algebraic_notation(
        &String::from("e2e4"),
        &game,
    ));

    assert_eq!(game.get_player_turn(), Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR"
    );

    // black moves
    game.make_move(&ChessMove::from_xboard_algebraic_notation(
        &String::from("e7e5"),
        &game,
    ));

    assert_eq!(game.get_player_turn(), Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR"
    );

    // white moves
    game.make_move(&ChessMove::from_xboard_algebraic_notation(
        &String::from("f1c4"),
        &game,
    ));

    assert_eq!(game.get_player_turn(), Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR"
    );
}

#[test]
fn test_castle() {
    // castle kingside
    let mut game = Game::from_fen_notation("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R".to_string());

    game.make_move(&ChessMove::from_xboard_algebraic_notation(
        &String::from("e1g1"),
        &game,
    ));

    assert_eq!(game.get_player_turn(), Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R4RK1"
    );

    game.make_move(&ChessMove::from_xboard_algebraic_notation(
        &String::from("e8g8"),
        &game,
    ));

    assert_eq!(game.get_player_turn(), Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "r4rk1/pppppppp/8/8/8/8/PPPPPPPP/R4RK1"
    );

    // castle queenside
    let mut game = Game::from_fen_notation("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R".to_string());

    game.make_move(&ChessMove::from_xboard_algebraic_notation(
        &String::from("e1c1"),
        &game,
    ));

    assert_eq!(game.get_player_turn(), Color::Black);
    assert_eq!(
        game.get_fen_notation(),
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R"
    );

    game.make_move(&ChessMove::from_xboard_algebraic_notation(
        &String::from("e8c8"),
        &game,
    ));

    assert_eq!(game.get_player_turn(), Color::White);
    assert_eq!(
        game.get_fen_notation(),
        "2kr3r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R"
    );
}

#[test]
fn test_pawn_promotion() {
    let mut game = Game::from_fen_notation("8/7P/8/8/8/8/8/8".to_string());

    game.make_move(&ChessMove::from_xboard_algebraic_notation(
        &String::from("h7h8q"),
        &game,
    ));

    assert_eq!(game.get_player_turn(), Color::Black);
    assert_eq!(game.get_fen_notation(), "7Q/8/8/8/8/8/8/8");
}
