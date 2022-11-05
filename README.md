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
