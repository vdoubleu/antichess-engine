use antichess_engine::chess_game::Game;

fn main() {
    // perft test by counting up total positions at a depth
    // these are all taken from the chess programming wiki

    // initial position
    // perft_test(
    //     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
    //     vec!(20, 400, 8902, 197281, 4865609),
    //     false,
    //     vec!(0, 0, 0, 0, 0)
    // );

    // position 2
    perft_test(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
        vec![48, 2039, 97862, 4085603],
        false,
        vec![2, 91, 3162, 128013],
    );

    // position 3
    // perft_test(
    //     "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8",
    //     vec!(14, 191, 2812, 43238, 674624),
    //     false,
    //     vec!(0, 0, 0, 0, 0),
    // );

    // position 4
    // perft_test(
    //     "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
    //     vec!(6, 264, 9467, 422333),
    //     false,
    //     vec!(0, 6, 0, 7795),
    // );

    // position 5
    // perft_test(
    //     "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
    //     vec!(44, 1486, 62379, 2103487),
    //     true,
    //     vec!(),
    // );

    // position 6
    // perft_test(
    //     "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1",
    //     vec!(46, 2079, 89890, 3894594),
    //     true,
    //     vec!(),
    // );
}

fn perft_test(
    fen: &str,
    expected_positions: Vec<usize>,
    check_only_total: bool,
    expected_castles: Vec<usize>,
) {
    let mut game = Game::from_fen_notation(fen).unwrap();

    println!("{}", game);

    let depth = expected_positions.len();

    let mut nodes_at_depth = vec![0; depth];
    let mut castle_at_depth = vec![0; depth];

    perft(
        &mut game,
        0,
        depth,
        &mut nodes_at_depth,
        &mut castle_at_depth,
    );

    for i in 0..depth {
        assert_eq!(
            nodes_at_depth[i], expected_positions[i],
            "nodes at depth {}",
            i
        );

        if !check_only_total {
            assert_eq!(
                castle_at_depth[i], expected_castles[i],
                "castles at depth {}",
                i
            );
        }
    }
}

fn perft(
    game: &mut Game,
    curr_depth: usize,
    depth: usize,
    nodes_at_depth: &mut Vec<usize>,
    castle_at_depth: &mut Vec<usize>,
) {
    if curr_depth == depth {
        return;
    }

    let moves = game.all_valid_moves_for_color_perft(game.player_turn);

    for m in moves {
        if m.is_castle(game) {
            castle_at_depth[curr_depth] += 1;
        }

        game.move_piece(&m);
        nodes_at_depth[curr_depth] += 1;
        perft(game, curr_depth + 1, depth, nodes_at_depth, castle_at_depth);
        game.unmove_move();
    }
}
