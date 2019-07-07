use failchain::{BoxedError, ChainErrorKind};
use failure::Fail;
use std::result::Result as StdResult;

pub type Error = BoxedError<ErrorKind>;
pub type Result<T> = StdResult<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Source buffer is too large (capacity > 2^32).")]
    SourceTooLarge,

    #[fail(
        display = "An unknown error caused the tokenizer to fail (the C function returned -1)."
    )]
    UnknownError,
}

impl ChainErrorKind for ErrorKind {
    type Error = Error;
}
