use std::string::FromUtf8Error;

use crate::shared::error::PackageReadError;

impl From<FromUtf8Error> for PackageReadError {
    fn from(error: FromUtf8Error) -> PackageReadError {
        PackageReadError(Box::new(error))
    }
}