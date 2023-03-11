use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("The Game Data is from an unsupported version.")]
    UnsupportedVersion,
    #[error("Image could not be rendered.")]
    ImageRender,
    #[error("The data seems to be corrupted.")]
    DataParse,
}
