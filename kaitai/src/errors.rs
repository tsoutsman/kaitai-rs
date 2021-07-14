use thiserror::Error;

pub type Result<T> = std::result::Result<T, KaitaiError>;

#[derive(Error, Debug)]
pub enum KaitaiError {
    #[error("end of stream reached, but no terminator {0} found")]
    EofBeforeTerminator(char),
    #[error("unexpected fixed contents got {actual:?}, was waiting for {expected:?}")]
    UnexpectedFileContents { actual: Vec<u8>, expected: Vec<u8> },
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
