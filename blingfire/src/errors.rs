use snafu::Snafu;
use std::result::Result as StdResult;

#[derive(Debug, Snafu, PartialEq)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Source buffer is too large (capacity > 2^32)."))]
    SourceTooLarge,

    #[snafu(display(
        "An unknown error caused the tokenizer to fail (the C function returned -1)."
    ))]
    UnknownError,
}

pub type Result<T, E = Error> = StdResult<T, E>;
