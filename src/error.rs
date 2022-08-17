use std::fmt;

use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub fn inner_path_already_exists<T: AsRef<str>>(inner_path: T) -> Error {
        Error::from(ErrorKind::InnerPathAlreadyExistsError(inner_path.as_ref().to_string()))
    }

    pub fn read_failed(start_pos: u64, failed_at: u64) -> Error {
        Error::from(ErrorKind::ReadBytesError { start_pos, failed_at })
    }

    pub fn io_error(source: std::io::Error) -> Error {
        Error::from(ErrorKind::IOError())
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::io_error(source)
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    InnerPathAlreadyExistsError(String),
    ReadBytesError {
        start_pos: u64,
        failed_at: u64
    },
    IOError(),
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::InnerPathAlreadyExistsError(ref inner_path) => {
                write!(f, "inner_path already exists: {}", inner_path)
            },
            ErrorKind::ReadBytesError { start_pos, failed_at } => {
                write!(f, "failed to read bytes: started at = {}, failed at = {}", start_pos, failed_at)
            },
            ErrorKind::IOError() => {
                write!(f, "I/O error")
            },
            ErrorKind::__Nonexhaustive => panic!("invalid error")
        }
    }
}
