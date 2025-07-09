use std::convert::Infallible;
use thiserror::Error;

#[derive(Error, Clone, Debug, Default)]
pub enum LibcError {
    #[error("there was no host name found")]
    GenericError(&'static str),

    // holy fuck this is shit
    #[error("str utf-8")]
    StrFromUtf8(#[from] std::str::Utf8Error),

    #[error("string utf-8")]
    StringFromUtf8(#[from] std::string::FromUtf8Error),

    #[error("HOW??")]
    FromInfallible(#[from] Infallible),

    #[default]
    #[error("Undefined error")]
    Undefined,

    #[error("Unknown error")]
    Unknown,
}

pub type LibcResult<T> = Result<T, LibcError>;
