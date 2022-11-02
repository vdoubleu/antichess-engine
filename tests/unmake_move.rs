use antichess_engine::chess_game::{ChessMove, Game, Color};

enum MoveDirection {
    Forward,
    Backward, 
    Split,
}

#[test]
fn test_unmake_move_baisc() {
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

#[test]
fn test_unmake_move_remake_repeat() {
    let mut game = Game::new_starting_game();

    let moves_strings: Vec<String> = vec![
        "e2e4".to_string(),
        "e7e5".to_string(),
        "g1f3".to_string(),
        "b8c6".to_string(),
        "f3e5".to_string(),
    ];

    let move_order: Vec<MoveDirection> = vec![
        MoveDirection::Forward,
        MoveDirection::Forward,
        MoveDirection::Backward,
        MoveDirection::Forward,
        MoveDirection::Backward,
        MoveDirection::Backward,
        MoveDirection::Forward,
        MoveDirection::Forward,
        MoveDirection::Forward,
    ];

    let mut move_store: Vec<ChessMove> = Vec::new();

    let mut curr_move_id = 0;
    for m in &move_order {
        match m {
            MoveDirection::Forward => {
                let curr_move = ChessMove::from_xboard_algebraic_notation(&moves_strings[curr_move_id], &game);
                println!("Making move: {:?}", curr_move);
                game.make_move(&curr_move);
                move_store.push(curr_move);
                curr_move_id += 1;
            }
            MoveDirection::Backward => {
                let curr_move = move_store.pop().unwrap();
                game.unmake_move(&curr_move);
                curr_move_id -= 1;
            }
            _ => {}
        } 
        println!("{} ", &game);
    }
}

#[test]
fn test_unmake_move_remake_repeat_2() {
    let mut game = Game::new_starting_game();

    let moves_strings: Vec<String> = vec![
        "a2a3".to_string(),
        "b8c6".to_string(),
        "a3a4".to_string(),
    ];

    let mut move_store: Vec<ChessMove> = Vec::new();
    let mut curr_move_id = 0;

    // white moves pawn
    let curr_move_1 = ChessMove::from_xboard_algebraic_notation(&moves_strings[curr_move_id], &game);
    game.make_move(&curr_move_1);
    move_store.push(curr_move_1);
    curr_move_id += 1;

    // black moves knight
    let curr_move_2 = ChessMove::from_xboard_algebraic_notation(&moves_strings[curr_move_id], &game);
    game.make_move(&curr_move_2);
    move_store.push(curr_move_2);
    curr_move_id += 1;

    // white moves pawn
    let curr_move_3 = ChessMove::from_xboard_algebraic_notation(&moves_strings[curr_move_id], &game);
    game.make_move(&curr_move_3);
    move_store.push(curr_move_3);
    curr_move_id += 1;

    // white undoes pawn move
    game.unmake_move(&curr_move_3);

    let white_moves = game.all_valid_moves_for_color(Color::White);

    assert!(white_moves.contains(&ChessMove::from_xboard_algebraic_notation(&"a3a4".to_string(), &game)));
    assert!(!white_moves.contains(&ChessMove::from_xboard_algebraic_notation(&"a3a5".to_string(), &game)));

    // black undoes knight move
    game.unmake_move(&curr_move_2);

    // black tries new knight move
    game.make_move(&ChessMove::from_xboard_algebraic_notation(&"b8a6".to_string(), &game));

    assert!(white_moves.contains(&ChessMove::from_xboard_algebraic_notation(&"a3a4".to_string(), &game)));
    assert!(!white_moves.contains(&ChessMove::from_xboard_algebraic_notation(&"a3a5".to_string(), &game)));

    //         MoveDirection::Backward => {
    //             let curr_move = move_store.pop().unwrap();
    //             game.unmake_move(&curr_move);
    //             curr_move_id -= 1;
    //         }
    //     println!("{} ", &game);
    // }
}
