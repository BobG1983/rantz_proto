use crate::prelude::*;
use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use std::{
    any::{type_name, TypeId},
    path::PathBuf,
};

#[derive(Default, Resource, Debug)]
pub struct ManifestLoader {
    manifests: HashMap<TypeId, ManifestStatus>,
    paths: HashMap<TypeId, PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestStatus {
    pub path: PathBuf,
    pub handle: UntypedHandle,
    pub load_state: LoadState,
}

impl ManifestLoader {
    /// Get an empty [ManifestLoader]
    pub fn new() -> Self {
        Default::default()
    }

    /// Registers a manifest to be loaded.
    pub fn register<M: Manifest>(&mut self, path: impl Into<PathBuf>) {
        let path: PathBuf = path.into();
        let type_id = std::any::TypeId::of::<M>();

        self.paths.insert(type_id, path);
    }

    /// Loads a registered manifest.
    pub fn load<M: Manifest>(&mut self, asset_server: &mut AssetServer) {
        let type_id = std::any::TypeId::of::<M>();
        let path = self.paths.get(&type_id);

        if let Some(path) = path {
            let handle: UntypedHandle = asset_server.load::<M>(path.clone()).untyped();

            self.manifests.insert(
                type_id,
                ManifestStatus {
                    path: path.clone(),
                    handle,
                    load_state: LoadState::Loading,
                },
            );

            self.paths.remove(&type_id);
        } else {
            error!("Manifest not loaded: {:?}", type_name::<M>());
        }
    }

    /// Registers a manifest to be loaded.
    pub fn register_collection<M: Manifest>(&mut self, path: impl Into<PathBuf>) {
        let path: PathBuf = path.into();
        let type_id = std::any::TypeId::of::<ManifestCollection<M>>();

        self.paths.insert(type_id, path);
    }

    /// Loads a registered manifest collection.
    pub fn load_collection<M: Manifest>(&mut self, asset_server: &mut AssetServer) {
        let type_id = std::any::TypeId::of::<ManifestCollection<M>>();
        let path = self.paths.get(&type_id);

        if let Some(path) = path {
            debug!("Loading Manifest Collection: {:?}", path);
            let handle: UntypedHandle = asset_server
                .load::<ManifestCollection<M>>(path.clone())
                .untyped();

            self.manifests.insert(
                type_id,
                ManifestStatus {
                    path: path.clone(),
                    handle,
                    load_state: LoadState::Loading,
                },
            );

            self.paths.remove(&type_id);
        } else {
            error!("Manifest Collection not loaded: {:?}", type_name::<M>());
        }
    }

    /// Updates the load state of all registered manifests.
    pub fn update_load_states(&mut self, asset_server: &AssetServer) {
        for status in self.manifests.values_mut() {
            status.load_state = asset_server
                .get_load_state(status.handle.clone_weak())
                .unwrap_or(LoadState::Failed);
        }
    }

    /// Returns the load state and other metadata for the given manifest.
    pub fn get_status<M: Manifest>(&self) -> Option<&ManifestStatus> {
        self.manifests.get(&std::any::TypeId::of::<M>())
    }

    /// Returns the load state and other metadata for the given manifest.
    pub fn get_collection_status<M: Manifest>(&self) -> Option<&ManifestStatus> {
        self.manifests
            .get(&std::any::TypeId::of::<ManifestCollection<M>>())
    }

    /// Removes the manifest from the loader.
    pub fn remove<M: Manifest>(&mut self) {
        self.manifests.remove(&std::any::TypeId::of::<M>());
        self.paths.remove(&std::any::TypeId::of::<M>());
    }

    /// Removes the manifest from the loader.
    pub fn remove_collection<M: Manifest>(&mut self) {
        self.manifests
            .remove(&std::any::TypeId::of::<ManifestCollection<M>>());
        self.paths
            .remove(&std::any::TypeId::of::<ManifestCollection<M>>());
    }

    /// Returns true if all registered manifests have loaded.
    pub fn all_loaded(&self) -> bool {
        self.manifests
            .values()
            .all(|status| status.load_state == LoadState::Loaded)
    }

    /// Returns true if any registered manifests have failed to load.
    pub fn any_failed(&self) -> bool {
        self.manifests
            .values()
            .any(|status| status.load_state == LoadState::Failed)
    }

    /// Iterates over all registered manifests.
    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &ManifestStatus)> {
        self.manifests.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.manifests.is_empty() && self.paths.is_empty()
    }
}
