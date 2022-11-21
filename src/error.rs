#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum ChessError {
    #[error("no start time set")]
    NoStartTime,
    #[error("no move generated")]
    NoMoveGenerated,
    #[error("no valid moves to try")]
    NoValidMoves,
}
