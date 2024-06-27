mod commands_ext;
mod id;
mod manifest_collection;
mod manifest_format;
mod manifest_loader;
mod manifest_trait;
mod proto_plugin;
mod prototype_library;
mod prototype_trait;
mod register;
mod schedule;
mod systems;

pub mod prelude {
    pub use crate::{id::Id, library::*, proto_plugin::*, schedules::*, traits::*};
    pub(crate) use crate::{loader::*, systems::*};
}

pub mod traits {
    pub use crate::{
        commands_ext::*, manifest_format::ManifestFormat, manifest_trait::Manifest,
        prototype_trait::Prototype, register::RegisterPrototype,
    };
}

pub mod library {
    pub use crate::prototype_library::{PrototypeLibrary, PrototypeLibraryError};
}

pub mod plugins {
    pub use crate::proto_plugin::ProtoPlugin;
}

pub mod schedules {
    pub use crate::schedule::ProtoSchedule;
}

pub(crate) mod loader {
    pub use crate::{manifest_collection::ManifestCollection, manifest_loader::ManifestLoader};
}
