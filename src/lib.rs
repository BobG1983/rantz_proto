mod commands_ext;
#[cfg(feature = "hot_reload")]
mod hot_reload;
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
    pub(crate) use crate::{
        commands_ext::ProtoSpawnTask,
        manifest_collection::ManifestCollection,
        manifest_loader::ManifestLoader,
        manifest_trait::AccessManifestFormat,
        systems::{handle_async_spawn, load, track_asset},
    };

    #[cfg(feature = "hot_reload")]
    pub(crate) use crate::hot_reload::*;

    pub use crate::{
        commands_ext::{SpawnPrototypeAsyncExt, SpawnPrototypeExt},
        id::Id,
        manifest_format::ManifestFormat,
        manifest_trait::Manifest,
        proto_plugin::ProtoPlugin,
        prototype_library::PrototypeLibrary,
        prototype_trait::Prototype,
        register::RegisterPrototype,
        schedule::ProtoSchedule,
    };
}
