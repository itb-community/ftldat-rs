pub use crate::shared::entry::PackageEntry;
pub use crate::shared::package::Package;
pub use crate::shared::reader::PackageReader;
pub use crate::shared::writer::PackageWriter;

mod shared;
mod dat;
mod pkg;

pub mod error {
    pub use crate::shared::error::*;
}
