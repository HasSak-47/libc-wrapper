use std::convert::Infallible;


macro_rules! into_error{
    ($error_ty: ty, $error_cont: tt) => {
        impl From<$error_ty> for LibcError{
            fn from(value: $error_ty) -> Self {
                Self::$error_cont(value)
            }
        }
    }
}

pub enum LibcError{
    GenericError(String),
    StrFromUtf8(std::str::Utf8Error),
    FromInfallible(Infallible)
}

into_error!(std::str::Utf8Error, StrFromUtf8);
into_error!(Infallible, FromInfallible);

pub type LibcrResult<T> = Result<T, LibcError>;
