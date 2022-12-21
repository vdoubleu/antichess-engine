# ANTI-CHESS ENGINE

A chess engine for anti-chess.  

## The Rules
If a person can take a piece, and it is their turn, they must take the piece. If you take the other person's king, you win.

## Running the code (Prod)
If you want to run the code for reals, and test it against other bots, or run it for evaluation, you have come to the right place. First, use the following command to compile the executable:

```
cargo build --release
```

You can then find the binary at `target/release/antichess-engine`  

To run this executable, you will need to pass in the color you would like the bot to play as. For example:
```
./target/release/antichess-engine white
```
or 
```
./target/release/antichess-engine black
```

---

## Running the code (Dev)
Within this repository, there are number executable to run. As a result, we have outlined the various runnable pieces of code below.  

NOTE: Running this code in the unoptimized debug mode without the `--release` flag is not suggested. It runs *really* slow if you do it like that, and it only takes marginally less time to compile.

### Playing against the bot (human vs bot)

By default, this is what happens when you simply run the program using the command:
```
cargo run --release -- white
```

The bot will play as white by default. But if you would like the bot to play as black, you can run the following command:
```
cargo run --release -- black
```
Of course you can pass in the flag for white (but this doesn't really do anything since it's already white by default).

If you would like to debug the program, you can enable debug logging by passing in an integer for the debug flag. 
```
cargo run --release -- --debug=1 white
```

For more information, you can use the help flag 
```
cargo run --release -- --help
```

### Running bot matches

To play bots against each other, run
```
python runmatch.py --white <WHITE_EXE_PATH> --black <BLACK_EXE_PATH>
```
Running just `python runmatch.py` without any arguments will use the bots in the release build.

You will probably want to capture stderr in a separate file by appending `2> <TEXT_FILE>` as to not show up with the output of the script.


### Running the opening book generator

Run the following command:
```
cargo run --release --bin opening-gen
```
to generate the most up to date opening book.

The repo should come with an existing up to date opening book, but if for some reason you need to recreate it, you can run that command.


### Running perft tests

Open up `perft/main.rs` to view the various perft tests. These are taking from the chess programming wiki. Comment and uncomment the ones you would like to run.  

To run them, run the following command:
```
cargo run --release --bin perft
```

### Running unit and integration tests

Simply run the command:
```
cargo test --release
```

## Citations

All external code are the Rust packages (called "crates") used in the engine listed in the file `Cargo.toml`, which have been cited below.

clap, Version 3.2.23. (2022). [Computer Software]. Retrieved from https://crates.io/crates/clap/3.2.23.

Fleischman, Stephen. (2019). pleco, Version 0.5.0 [Computer software]. Retrieved from https://crates.io/crates/pleco.

rand, Version 0.8.4. (2021). [Computer Software]. Retrieved from https://crates.io/crates/rand/0.8.4.

Tolnay, David. (2022). anyhow, Version 1.0 [Computer software]. Retrieved from https://crates.io/crates/anyhow.

Tolnay, David. (2022). thiserror, Version 1.0 [Computer Software]. Retrieved from https://crates.io/crates/thiserror/.
