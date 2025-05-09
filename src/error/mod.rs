use std::convert::Infallible;
use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum StringErrors{
}

#[derive(Error, Clone, Debug, Default)]
pub enum LibcError{
    #[error("there was no host name found")]
    GenericError(&'static str),

    // holy fuck this is shit
    #[error("str utf-8")]
    StrFromUtf8 (#[from] std::str::Utf8Error),

    #[error("string utf-8")]
    StringFromUtf8(#[from] std::string::FromUtf8Error),

    #[error("char* to String/str is weird")]
    CharPtrToRustBAD,

    #[error("HOW??")]
    FromInfallible(#[from] Infallible),


    #[error("there was no host name found")]
    NoHostName,

    #[default]
    #[error("Undefined error")]
    Undefined,

    #[error("Unknown error")]
    Unknown,

    #[error("No {0} returned null")]
    ReturnedNull(String)

}

pub type LibcResult<T> = Result<T, LibcError>;
