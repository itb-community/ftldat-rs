use thiserror::Error;

#[derive(Error, Debug)]
#[error("inner path '{0}' already exists within the package")]
pub struct InnerPathAlreadyExistsError(pub(crate) String);