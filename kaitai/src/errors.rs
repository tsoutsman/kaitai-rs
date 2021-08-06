use thiserror::Error;

/// The type returned by [`KaitaiStruct`] functions.
pub type Result<T> = std::result::Result<T, KaitaiError>;

/// Enum representing the potential errors emitted by this crate.
#[derive(Debug, Error)]
pub enum KaitaiError {
    /// Returned by the [`read_byte_term`] function in [`KaitaiStream`] when the cursor reaches the end
    /// of the buffer before the terminator is reached. This should not necessarily be treated as an
    /// error but it should be differentiated from an [`IoError`].
    #[error("end of stream reached, but no terminator {0} found")]
    EofBeforeTerminator(char),

    /// Returned by the `ensure_fixed_contents` function in `KaitaiStream` when the contents of the
    /// file don't match the expected value.
    #[error("unexpected fixed contents got {actual:?}, was expecting {expected:?}")]
    UnexpectedContents {
        /// The actual value read in
        actual: Vec<u8>,
        /// The expected value
        expected: Vec<u8>,
    },

    /// A generic IO error.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
