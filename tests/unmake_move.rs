use antichess_engine::chess_game::{ChessMove, Color, Game};

use anyhow::{Context, Result};

enum MoveDirection {
    Forward,
    Backward,
}

#[test]
fn test_unmove_piece_basic() -> Result<()> {
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

    let mut undo_count = 0;

    for m in &moves_strings {
        let curr_move = ChessMove::from_xboard_algebraic_notation(&m)?;
        game.move_piece(&curr_move)?;
        undo_count += 1;
    }

    for _ in 0..undo_count {
        game.unmove_move().context("Failed to unmove move")?;
    }

    assert_eq!(
        game.get_fen_notation(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"
    );

    Ok(())
}

#[test]
fn test_unmove_piece_remake_repeat() -> Result<()> {
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

    let mut curr_move_id = 0;
    for m in &move_order {
        match m {
            MoveDirection::Forward => {
                let curr_move =
                    ChessMove::from_xboard_algebraic_notation(&moves_strings[curr_move_id])?;
                println!("Making move: {:?}", curr_move);
                game.move_piece(&curr_move)?;
                curr_move_id += 1;
            }
            MoveDirection::Backward => {
                game.unmove_move().context("Failed to unmove move")?;
                curr_move_id -= 1;
            }
        }
        eprintln!("{} ", &game);
    }

    Ok(())
}

#[test]
fn test_unmove_piece_remake_repeat_2() -> Result<()> {
    let mut game = Game::new_starting_game();

    let moves_strings: Vec<String> =
        vec!["a2a3".to_string(), "b8c6".to_string(), "a3a4".to_string()];

    let mut curr_move_id = 0;

    // white moves pawn
    let curr_move_1 = ChessMove::from_xboard_algebraic_notation(&moves_strings[curr_move_id])?;
    game.move_piece(&curr_move_1)?;
    curr_move_id += 1;

    // black moves knight
    let curr_move_2 = ChessMove::from_xboard_algebraic_notation(&moves_strings[curr_move_id])?;
    game.move_piece(&curr_move_2)?;
    curr_move_id += 1;

    // white moves pawn
    let curr_move_3 = ChessMove::from_xboard_algebraic_notation(&moves_strings[curr_move_id])?;

    game.move_piece(&curr_move_3)?;

    // white undoes pawn move
    game.unmove_move().context("Failed to unmove move")?;

    let white_moves = game.all_valid_moves_for_color(Color::White);

    assert!(
        white_moves.contains(&ChessMove::from_xboard_algebraic_notation(
            &"a3a4".to_string()
        )?)
    );
    assert!(
        !white_moves.contains(&ChessMove::from_xboard_algebraic_notation(
            &"a3a5".to_string()
        )?)
    );

    // black undoes knight move
    game.unmove_move().context("Failed to unmove move")?;

    // black tries new knight move
    game.move_piece(&ChessMove::from_xboard_algebraic_notation(
        &"b8a6".to_string(),
    )?)?;

    assert!(
        white_moves.contains(&ChessMove::from_xboard_algebraic_notation(
            &"a3a4".to_string()
        )?)
    );
    assert!(
        !white_moves.contains(&ChessMove::from_xboard_algebraic_notation(
            &"a3a5".to_string()
        )?)
    );

    Ok(())
}

#[test]
fn test_pawn_promotion_while_take_then_undo() -> Result<()> {
    let mut game = Game::from_fen_notation("6n1/7P/8/8/8/8/8/8")?;

    game.move_piece(&ChessMove::from_xboard_algebraic_notation("h7g8r")?)?;

    assert_eq!(game.player_turn, Color::Black);
    assert_eq!(game.get_fen_notation(), "6R1/8/8/8/8/8/8/8 b");

    game.unmove_move().context("Failed to unmove move")?;

    assert_eq!(game.player_turn, Color::White);
    assert_eq!(game.get_fen_notation(), "6n1/7P/8/8/8/8/8/8 w");

    Ok(())
}
