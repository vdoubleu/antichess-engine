import chess
import subprocess
from re import search

def is_game_over(line):
  return search("^(Game over.|No moves available$|error:)", line)

if __name__ == "__main__":
  # TODO: also make it possible to specify as arguments
  engine_white_executable = "./target/release/antichess-engine.exe"
  engine_black_executable = "./target/release/antichess-engine.exe"

  engine_white = subprocess.Popen(
    [engine_white_executable, "white"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    stdin=subprocess.PIPE,
  )

  engine_black = subprocess.Popen(
    [engine_black_executable, "black"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    stdin=subprocess.PIPE,
  )

  board = chess.Board()

  print("Game started:")
  print(board.unicode(invert_color=True, borders=True), "\n")

  while not board.is_game_over():
    white_move = engine_white.stdout.readline()
    # print("WHITE: ", white_move)
    engine_black.stdin.write(bytes(white_move))
    engine_black.stdin.flush()
    engine_black.stderr.flush()
    board.push_xboard(white_move.decode().rstrip())

    print(f'White played {white_move.decode().rstrip()}:')
    print(board.unicode(invert_color=True, borders=True), "\n")

    black_move = engine_black.stdout.readline()
    # print("BLACK: ", black_move)
    engine_white.stdin.write(bytes(black_move))
    engine_white.stdin.flush()
    engine_white.stderr.flush()
    if not board.is_game_over():
      board.push_xboard(black_move.decode().rstrip())

      print(f'Black played {black_move.decode().rstrip()}:')
      print(board.unicode(invert_color=True, borders=True), "\n")


  print(board.outcome().result())

  engine_white.kill()
  engine_black.kill()