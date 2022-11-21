import argparse
import subprocess
from re import search

parser = argparse.ArgumentParser(description='Run a match between two bots.')
parser.add_argument("--white",
                     type=str,
                     help="path to executable for bot to play White",
                     default="./target/release/antichess-engine.exe")
parser.add_argument("--black",
                     type=str,
                     help="path to executable for bot to play Black",
                     default="./target/release/antichess-engine.exe")

def is_game_over(line):
  return search("^((Game over.)|(No moves available)|(error:))", line)

if __name__ == "__main__":
  args = parser.parse_args()
  engine_white_executable = args.white
  engine_black_executable = args.black

  engine_white = subprocess.Popen(
    [engine_white_executable, "white"],
    stdout=subprocess.PIPE,
    # stderr=subprocess.PIPE,
    stdin=subprocess.PIPE,
  )

  engine_black = subprocess.Popen(
    [engine_black_executable, "black"],
    stdout=subprocess.PIPE,
    # stderr=subprocess.PIPE,
    stdin=subprocess.PIPE,
  )

  print("Game started")

  while True:
    white_move = engine_white.stdout.readline()
    if is_game_over(white_move.decode()):
      print(white_move.decode())
      break

    engine_black.stdin.write(bytes(white_move))
    engine_black.stdin.flush()
    # engine_black.stderr.flush()

    print(f'White: {white_move.decode()}')

    black_move = engine_black.stdout.readline()
    if is_game_over(black_move.decode()):
      print(black_move.decode())
      break

    engine_white.stdin.write(bytes(black_move))
    engine_white.stdin.flush()
    # engine_white.stderr.flush()

    print(f'Black: {black_move.decode()}')

  # engine_white.kill()
  # engine_black.kill()
