mod id;
mod manifest_trait;
mod prototype_library;
mod prototype_trait;

pub mod prelude {
    pub use crate::id::Id;
    pub use crate::manifest_trait::Manifest;
    pub use crate::prototype_library::{LibraryError, PrototypeLibrary};
    pub use crate::prototype_trait::Prototype;
}
