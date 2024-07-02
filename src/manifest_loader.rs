use crate::prelude::*;
use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use std::{
    any::{type_name, TypeId},
    path::PathBuf,
};

#[derive(Default, Resource, Debug)]
pub struct ManifestLoader {
    to_process: HashMap<TypeId, ManifestStatus>,
    to_load: HashMap<TypeId, PathBuf>,
    processed: HashMap<TypeId, ManifestStatus>,
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
        let type_id = std::any::TypeId::of::<ManifestCollection<M>>();

        self.to_load.insert(type_id, path);
    }

    /// Loads a registered manifest collection.
    pub fn load<M: Manifest>(&mut self, asset_server: &mut AssetServer) {
        let type_id = std::any::TypeId::of::<ManifestCollection<M>>();
        let path = self.to_load.get(&type_id);

        if let Some(path) = path {
            debug!("Loading Manifest Collection: {:?}", path);
            let handle: UntypedHandle = asset_server
                .load::<ManifestCollection<M>>(path.clone())
                .untyped();

            self.to_process.insert(
                type_id,
                ManifestStatus {
                    path: path.clone(),
                    handle,
                    load_state: LoadState::Loading,
                },
            );

            self.to_load.remove(&type_id);
        } else {
            error!("Manifest Collection not loaded: {:?}", type_name::<M>());
        }
    }

    pub fn process<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        assets: &mut Assets<ManifestCollection<M>>,
        protos: &mut PrototypeLibrary<P>,
    ) {
        let type_id = std::any::TypeId::of::<ManifestCollection<M>>();
        let status = self.to_process.get(&type_id);
        let mut handled = false;
        if let Some(status) = status {
            match status.load_state {
                LoadState::Loaded => {
                    debug!("Manifest loaded: {:?}", type_name::<M>());
                    let handle = status.handle.clone_weak().typed::<ManifestCollection<M>>();
                    if let Some(manifest) = assets.get(handle) {
                        for m in manifest {
                            let item: P = m.reify();
                            handled = true;
                            if let Err(e) = protos.insert(item) {
                                error!("Failed to insert prototype: {:?}", e);
                            }
                        }
                    }
                }
                LoadState::Failed => {
                    error!("Manifest failed to load: {:?}", type_name::<M>());
                }
                LoadState::NotLoaded => {
                    debug!("Manifest not loaded: {:?}", type_name::<M>());
                }
                LoadState::Loading => {
                    debug!("Manifest loading: {:?}", type_name::<M>());
                }
            }
        }

        if handled {
            let status = status.unwrap().clone();
            self.to_process.remove(&type_id);
            self.processed.insert(type_id, status);
        }
    }

    /// Updates the load state of all registered manifests.
    pub fn update_load_states(&mut self, asset_server: &AssetServer) {
        for status in self.to_process.values_mut() {
            status.load_state = asset_server
                .get_load_state(status.handle.clone_weak())
                .unwrap_or(LoadState::Failed);
        }
    }

    /// Returns the load state and other metadata for the given manifest.
    pub fn get<M: Manifest>(&self) -> Option<&ManifestStatus> {
        self.to_process.get(&std::any::TypeId::of::<M>())
    }

    /// Returns the load state and other metadata for the given manifest.
    pub fn get_collection<M: Manifest>(&self) -> Option<&ManifestStatus> {
        self.to_process
            .get(&std::any::TypeId::of::<ManifestCollection<M>>())
    }

    pub fn is_empty(&self) -> bool {
        self.to_process.is_empty() && self.to_load.is_empty()
    }
}
