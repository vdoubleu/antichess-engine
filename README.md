# ANTI-CHESS ENGINE

A chess engine for anti-chess.  

## The Rules
If a person can take a piece, and it is their turn, they must take the piece. If you take the other person's king, you win.

## Running bot matches

To play bots against each other, run
```
python runmatch.py --white <WHITE_EXE_PATH> --black <BLACK_EXE_PATH>
```
Running just `python runmatch.py` without any arguments will use the bots in the release build.

You will probably want to capture stderr in a separate file by appending `2> <TEXT_FILE>` as to not show up with the output of the script.

## Running the opening book generator

Run 
```
cargo run --bin opening-gen
```
to generate the most up to date opening book.

The repo should come with an existing up to date opening book, but if for some reason you need to recreate it, you can run that command.


## TODO list
- when error encountered for move gen, try gen on smaller depth, or resort to random move
- handle error in piece move

- perft sketchy en passant pos

- king safety evaluation

- dynamic search depth / search time

- gen opening book for first moves for either side

- 50 move draw, draw by repetition
