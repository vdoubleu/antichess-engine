use crate::chess_game::{ChessMove, Pos};

#[derive(thiserror::Error, Debug)]
pub enum ChessError {
    #[error("invalid move: {0}")]
    InvalidMove(ChessMove),
    #[error("no start time set")]
    NoStartTime,
    #[error("invalid piece character {0}")]
    InvalidPieceChar(char),
    #[error("cannot relocate, no piece at pos {0}")]
    NoPieceAtPos(Pos),
    #[error("no move to undo")]
    NoMoveToUndo,
    #[error("no move generated")]
    NoMoveGenerated,
    #[error("no valid moves to try")]
    NoValidMoves,

    #[error("no rook at expected column(s)")]
    NoRookAtExpectedColumn,
}
