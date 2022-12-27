mod shared;
pub mod dat;
pub mod pkg;

pub(crate) use crate::shared::entry::Entry as Entry;
pub use crate::shared::package::Package as Package;

pub mod error {
    pub use crate::shared::error::*;
}
