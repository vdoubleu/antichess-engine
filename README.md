# ANTI-CHESS ENGINE

A chess engine for anti-chess.  

## The Rules
If a person can take a piece, and it is their turn, they must take the piece. If you take the other person's king, you win.

## Running the code
Within this repository, there are number executable to run. As a result, we have outlined the various runnable pieces of code below.  

NOTE: Running this code in the unoptimized debug mode is not suggested. It runs *really* slow if you do it like that, and it only takes marginally less time to compile.

### Playing against the bot (human vs bot)

By default, this is what happens when you simply run the program using the command:
```
cargo run --release
```

The bot will play as white by default. But if you would like the bot to play as black instead, you can run the following command:
```
cargo run --release -- black
```
Of course you can pass in the flag for white (but this doesn't really do anything since it's already white by default).


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

Everything should pass. If something doesn't pass, then go fix it.


## TODO list
- when error encountered for move gen, try gen on smaller depth, or resort to random move
- handle error in piece move

- perft sketchy en passant pos

- king safety evaluation

- dynamic search depth / search time

- gen opening book for first moves for either side

- 50 move draw, draw by repetition
