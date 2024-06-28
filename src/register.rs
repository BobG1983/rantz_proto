use crate::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

pub trait RegisterPrototype {
    fn add_prototype<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> &mut Self;

    fn add_prototype_collection<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> &mut Self;
}

impl RegisterPrototype for App {
    fn add_prototype<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> &mut Self {
        // Init Assets & Resources
        self.init_asset::<M>();
        self.init_resource::<PrototypeLibrary<P>>();

        // Add child plugins for asset loading
        add_asset_plugins::<M>(self);

        // Register the manifest for loading, creating a loader if there isn't one
        self.world
            .resource_scope(|world: &mut World, mut asset_server: Mut<AssetServer>| {
                check_insert_loader(world);
                world
                    .resource_mut::<ManifestLoader>()
                    .register::<M>(path, asset_server.as_mut());
            });

        // Add processing system
        self.add_systems(
            PreUpdate,
            process_manifest::<M, P>.in_set(ProtoSchedule::Processing),
        );

        self
    }

    fn add_prototype_collection<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> &mut Self {
        // Init Assets & Resources
        self.init_asset::<M>();
        self.init_resource::<PrototypeLibrary<P>>();

        // Add child plugins for asset loading
        add_asset_plugins::<ManifestCollection<M>>(self);

        // Register the manifest for loading, creating a loader if there isn't one
        self.world
            .resource_scope(|world: &mut World, mut asset_server: Mut<AssetServer>| {
                check_insert_loader(world);
                world
                    .resource_mut::<ManifestLoader>()
                    .register::<M>(path, asset_server.as_mut());
            });

        // Add processing system
        self.add_systems(
            PreUpdate,
            process_manifest_collection::<M, P>.in_set(ProtoSchedule::Processing),
        );

        self
    }
}

fn add_asset_plugins<T: AccessManifestFormat + Clone + Asset + for<'de> Deserialize<'de>>(
    app: &mut App,
) {
    match T::manifest_format() {
        #[cfg(feature = "ron")]
        ManifestFormat::Ron => {
            app.add_plugins(bevy_common_assets::ron::RonAssetPlugin::<T>::new(&[]));
        }
        #[cfg(feature = "json")]
        ManifestFormat::Json => {
            app.add_plugins(bevy_common_assets::json::JsonAssetPlugin::<T>::new(&[]));
        }
        #[cfg(feature = "yaml")]
        ManifestFormat::Yaml => {
            app.add_plugins(bevy_common_assets::yaml::YamlAssetPlugin::<T>::new(&[]));
        }
        #[cfg(feature = "toml")]
        ManifestFormat::Toml => {
            app.add_plugins(bevy_common_assets::toml::TomlAssetPlugin::<T>::new(&[]));
        }
        #[cfg(feature = "csv")]
        ManifestFormat::Csv => {
            app.add_plugins(bevy_common_assets::csv::CsvAssetPlugin::<T>::new(&[]));
        }
        #[cfg(feature = "xml")]
        ManifestFormat::Xml => {
            app.add_plugins(bevy_common_assets::xml::XmlAssetPlugin::<T>::new(&[]));
        }
        #[cfg(feature = "msgpack")]
        ManifestFormat::MsgPack => {
            app.add_plugins(bevy_common_assets::msgpack::MsgPackAssetPlugin::<T>::new(
                &[],
            ));
        }
        ManifestFormat::Custom => (), // Users must register their own asset loader for custom formats.
    }
}

fn check_insert_loader(world: &mut World) {
    if world.get_resource::<ManifestLoader>().is_none() {
        let loader = ManifestLoader::new();
        world.insert_resource(loader);
    }
}
