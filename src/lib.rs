mod shared;
pub mod dat;
pub mod pkg;

pub use crate::shared::package::Package as Package;
pub use crate::shared::entry::PackageEntry as PackageEntry;

pub mod error {
    pub use crate::shared::error::*;
}
