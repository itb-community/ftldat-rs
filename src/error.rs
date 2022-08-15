use crate::error::Error::NestedError;

#[derive(Debug)]
pub enum Error {
    InnerPathAlreadyExists {
        path: String,
    },
    NestedError {
        error: std::io::Error
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        NestedError { error: err }
    }
}
