mod shared;
pub mod dat_package;
pub mod pkg_package;

pub(crate) use crate::shared::entry::Entry as Entry;
pub use crate::shared::package::Package as Package;

pub mod error {
    pub use crate::shared::error::shared::*;
    pub use crate::shared::error::reader::*;
    pub use crate::shared::error::writer::*;
}
