use snafu::Snafu;
use std::result::Result as StdResult;

/// Error enum encoding tokenization errors.
#[derive(Debug, Snafu, PartialEq)]
#[snafu(visibility = "pub")]
pub enum Error {
    /// Source buffer is too large (capacity > MAX_TEXT_LENGTH).
    #[snafu(display("Source buffer is too large (capacity > {}).", max_text_length))]
    SourceTooLarge { max_text_length: usize },

    /// An unknown error caused the tokenizer to fail (the C++ function returned -1).
    #[snafu(display(
        "An unknown error caused the tokenizer to fail (the C++ function returned -1)."
    ))]
    UnknownError,
}

/// Result of calling the tokenizer functions.
pub type Result<T, E = Error> = StdResult<T, E>;
