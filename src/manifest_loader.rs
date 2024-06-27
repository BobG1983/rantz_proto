use crate::prelude::*;
use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use std::{any::TypeId, path::PathBuf};

#[derive(Default, Resource, Debug)]
pub struct ManifestLoader {
    manifests: HashMap<TypeId, ManifestStatus>,
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
    pub fn register<M: Manifest>(
        &mut self,
        path: impl Into<PathBuf>,
        asset_server: &mut AssetServer,
    ) {
        let path: PathBuf = path.into();

        let handle: UntypedHandle = asset_server.load::<M>(path.clone()).untyped();
        let type_id = std::any::TypeId::of::<M>();

        self.manifests.insert(
            type_id,
            ManifestStatus {
                path: path.clone(),
                handle,
                load_state: LoadState::Loading,
            },
        );
    }

    /// Registers a manifest to be loaded.
    pub fn register_collection<M: Manifest>(
        &mut self,
        path: impl Into<PathBuf>,
        asset_server: &mut AssetServer,
    ) {
        let path: PathBuf = path.into();

        let handle: UntypedHandle = asset_server
            .load::<ManifestCollection<M>>(path.clone())
            .untyped();
        let type_id = std::any::TypeId::of::<M>();

        self.manifests.insert(
            type_id,
            ManifestStatus {
                path: path.clone(),
                handle,
                load_state: LoadState::Loading,
            },
        );
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
    pub fn get_manifest_status<M: Manifest>(&self) -> Option<&ManifestStatus> {
        self.manifests.get(&std::any::TypeId::of::<M>())
    }

    /// Returns true if all registered manifests have loaded.
    pub fn all_manifests_loaded(&self) -> bool {
        self.manifests
            .values()
            .all(|status| status.load_state == LoadState::Loaded)
    }

    /// Returns true if any registered manifests have failed to load.
    pub fn any_manifests_failed(&self) -> bool {
        self.manifests
            .values()
            .any(|status| status.load_state == LoadState::Failed)
    }

    /// Iterates over all registered manifests.
    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &ManifestStatus)> {
        self.manifests.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.manifests.is_empty()
    }
}
