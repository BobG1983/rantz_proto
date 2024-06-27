use crate::prelude::*;
use bevy::prelude::*;
use std::path::PathBuf;

pub trait RegisterPrototype {
    fn add_prototype<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> &mut Self;

    fn add_prototypes<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> &mut Self;
}

impl RegisterPrototype for App {
    fn add_prototype<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> &mut Self {
        // init assets
        self.init_asset::<M>();
        self.init_resource::<PrototypeLibrary<P>>();

        // add loading systems
        self.add_systems(
            PreUpdate,
            process_manifest::<M, P>.in_set(ProtoSchedule::Processing),
        );

        // add child plugins
        match M::FORMAT {
            #[cfg(feature = "ron")]
            ManifestFormat::Ron => {
                self.add_plugins(bevy_common_assets::ron::RonAssetPlugin::<M>::new(&[]));
            }
            #[cfg(feature = "json")]
            ManifestFormat::Json => {
                self.add_plugins(bevy_common_assets::json::JsonAssetPlugin::<M>::new(&[]));
            }
            #[cfg(feature = "yaml")]
            ManifestFormat::Yaml => {
                self.add_plugins(bevy_common_assets::yaml::YamlAssetPlugin::<M>::new(&[]));
            }
            #[cfg(feature = "toml")]
            ManifestFormat::Toml => {
                self.add_plugins(bevy_common_assets::toml::TomlAssetPlugin::<M>::new(&[]));
            }
            #[cfg(feature = "csv")]
            ManifestFormat::Csv => {
                self.add_plugins(bevy_common_assets::csv::CsvAssetPlugin::<M>::new(&[]));
            }
            #[cfg(feature = "xml")]
            ManifestFormat::Xml => {
                self.add_plugins(bevy_common_assets::xml::XmlAssetPlugin::<M>::new(&[]));
            }
            #[cfg(feature = "msgpack")]
            ManifestFormat::MsgPack => {
                self.add_plugins(bevy_common_assets::msgpack::MsgPackAssetPlugin::<M>::new(
                    &[],
                ));
            }
            ManifestFormat::Custom => (), // Users must register their own asset loader for custom formats.
        }

        // Finally, register the asset for loading
        self.world
            .resource_scope(|world, mut asset_server: Mut<AssetServer>| {
                let mut manifest_loader = world.resource_mut::<ManifestLoader>();
                manifest_loader.register::<M>(path, asset_server.as_mut());
            });

        self
    }

    fn add_prototypes<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> &mut Self {
        // init assets
        self.init_asset::<ManifestCollection<M>>();
        self.init_resource::<PrototypeLibrary<P>>();

        // add loading systems
        self.add_systems(
            PreUpdate,
            process_manifest_collection::<M, P>.in_set(ProtoSchedule::Processing),
        );

        // add child plugins
        match M::FORMAT {
            #[cfg(feature = "ron")]
            ManifestFormat::Ron => {
                self.add_plugins(bevy_common_assets::ron::RonAssetPlugin::<
                    ManifestCollection<M>,
                >::new(&[]));
            }
            #[cfg(feature = "json")]
            ManifestFormat::Json => {
                self.add_plugins(bevy_common_assets::json::JsonAssetPlugin::<
                    ManifestCollection<M>,
                >::new(&[]));
            }
            #[cfg(feature = "yaml")]
            ManifestFormat::Yaml => {
                self.add_plugins(bevy_common_assets::yaml::YamlAssetPlugin::<
                    ManifestCollection<M>,
                >::new(&[]));
            }
            #[cfg(feature = "toml")]
            ManifestFormat::Toml => {
                self.add_plugins(bevy_common_assets::toml::TomlAssetPlugin::<
                    ManifestCollection<M>,
                >::new(&[]));
            }
            #[cfg(feature = "csv")]
            ManifestFormat::Csv => {
                self.add_plugins(bevy_common_assets::csv::CsvAssetPlugin::<
                    ManifestCollection<M>,
                >::new(&[]));
            }
            #[cfg(feature = "xml")]
            ManifestFormat::Xml => {
                self.add_plugins(bevy_common_assets::xml::XmlAssetPlugin::<
                    ManifestCollection<M>,
                >::new(&[]));
            }
            #[cfg(feature = "msgpack")]
            ManifestFormat::MsgPack => {
                self.add_plugins(bevy_common_assets::msgpack::MsgPackAssetPlugin::<
                    ManifestCollection<M>,
                >::new(&[]));
            }
            ManifestFormat::Custom => (), // Users must register their own asset loader for custom formats.
        }

        // Finally, register the asset for loading
        self.world
            .resource_scope(|world, mut asset_server: Mut<AssetServer>| {
                let mut manifest_loader = world.resource_mut::<ManifestLoader>();
                manifest_loader.register_collection::<M>(path, asset_server.as_mut());
            });

        self
    }
}
