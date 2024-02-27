use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinesweeperError {
    #[error("Gif Encoding Error")]
    GifEncoding,
    #[error("Image insertion Error")]
    ImageInsertion,
    #[error("No frames error")]
    NoFrames,
    #[error("Unable to parse API Data")]
    ApiDataParse,
    #[error("Gamedata not found")]
    GameDataNotFound,
    #[error("Data could not be parsed")]
    DataParseError,
    #[error("No Api Key was found for the provider")]
    ApiKeyNotFound,
}
