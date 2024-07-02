use crate::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

pub trait RegisterPrototype {
    fn add_prototype<M: Manifest<Output = P>, P: Prototype>(
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
        check_insert_loader(&mut self.world);
        let mut loader = self.world.resource_mut::<ManifestLoader>();
        loader.register::<M>(path);

        // Add child plugins for asset loading
        add_plugins::<ManifestCollection<M>>(self);

        // Add processing system
        self.add_systems(
            Update,
            (load::<M>, track_asset::<M, P>)
                .chain()
                .in_set(ProtoSchedule::Loading),
        );

        // If hot reloading then register the rebuild system
        #[cfg(feature = "hot_reload")]
        register_rebuild_system::<P>(&mut self.world);

        self
    }
}

fn add_plugins<T: AccessManifestFormat + Clone + Asset + for<'de> Deserialize<'de>>(app: &mut App) {
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
