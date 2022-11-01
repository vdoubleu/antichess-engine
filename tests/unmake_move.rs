use antichess_engine::chess_game::{ChessMove, Game};

#[test]
fn test_unmake_move() {
    let mut game = Game::new_starting_game();

    let moves_strings: Vec<String> = vec![
        "e2e4".to_string(),
        "e7e5".to_string(),
        "g1f3".to_string(),
        "b8c6".to_string(),
        "f3e5".to_string(),
        "c6e5".to_string(),
        "f1d3".to_string(),
        "d7d6".to_string(),
        "e1g1".to_string(),
    ];

    let mut move_store: Vec<ChessMove> = Vec::new();

    for m in &moves_strings {
        let curr_move = ChessMove::from_xboard_algebraic_notation(&m, &game);
        game.make_move(&curr_move);
        move_store.push(curr_move);
    }

    for m in move_store.iter().rev() {
        game.unmake_move(&m);
    }

    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );
}
