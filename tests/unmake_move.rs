// use antichess_engine::chess_game::{ChessMove, Color, Game};

use pleco::{board::FenBuildError, Board, Player};

use anyhow::Result;

enum MoveDirection {
    Forward,
    Backward,
}

#[test]
fn test_unmove_piece_basic() -> Result<()> {
    let mut game = Board::start_pos();

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

    let mut undo_count = 0;

    for m in &moves_strings {
        game.apply_uci_move(m);
        undo_count += 1;
    }

    for _ in 0..undo_count {
        game.undo_move();
    }

    assert_eq!(
        game.fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );

    Ok(())
}

#[test]
fn test_unmove_piece_remake_repeat() -> Result<()> {
    let mut game = Board::start_pos();

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

    let mut curr_move_id = 0;
    for m in &move_order {
        match m {
            MoveDirection::Forward => {
                game.apply_uci_move(moves_strings[curr_move_id].as_str());
                curr_move_id += 1;
            }
            MoveDirection::Backward => {
                game.undo_move();
                curr_move_id -= 1;
            }
        }
        eprintln!("{} ", &game);
    }

    Ok(())
}

#[test]
fn test_unmove_piece_remake_repeat_2() -> Result<()> {
    let mut game = Board::start_pos();

    let moves_strings: Vec<String> =
        vec!["a2a3".to_string(), "b8c6".to_string(), "a3a4".to_string()];

    let mut curr_move_id = 0;

    // white moves pawn
    game.apply_uci_move(moves_strings[curr_move_id].as_str());
    curr_move_id += 1;

    // black moves knight
    game.apply_uci_move(moves_strings[curr_move_id].as_str());
    curr_move_id += 1;

    // white moves pawn
    game.apply_uci_move(moves_strings[curr_move_id].as_str());

    // white undoes pawn move
    game.undo_move();

    let white_moves = game.generate_moves();

    assert!(white_moves
        .iter()
        .find(|x| x.stringify() == "a3a4")
        .is_some(),);
    assert!(white_moves
        .iter()
        .find(|x| x.stringify() == "a3a5")
        .is_none(),);

    // black undoes knight move
    game.undo_move();

    // black tries new knight move
    game.apply_uci_move("b8a6");

    assert!(white_moves
        .iter()
        .find(|x| x.stringify() == "a3a4")
        .is_some(),);
    assert!(white_moves
        .iter()
        .find(|x| x.stringify() == "a3a5")
        .is_none(),);

    Ok(())
}

#[test]
fn test_pawn_promotion_while_take_then_undo() -> Result<(), FenBuildError> {
    let mut game = Board::from_fen("6n1/k6P/8/8/8/8/8/7K w - - 0 1")?;

    game.apply_uci_move("h7g8r");

    assert_eq!(game.turn(), Player::Black);
    assert_eq!(game.fen(), "6R1/k7/8/8/8/8/8/7K b - - 0 1");

    game.undo_move();

    assert_eq!(game.turn(), Player::White);
    assert_eq!(game.fen(), "6n1/k6P/8/8/8/8/8/7K w - - 0 1");

    Ok(())
}
