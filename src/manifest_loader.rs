use crate::prelude::*;
use bevy::{prelude::*, utils::HashMap};
#[cfg(feature = "progress_tracking")]
use iyes_progress::Progress;
use std::{
    any::{type_name, TypeId},
    path::PathBuf,
};

#[derive(Default, Resource, Debug)]
pub struct ManifestLoader {
    to_load: HashMap<TypeId, PathBuf>,
    loaded: HashMap<TypeId, UntypedHandle>,
    processed: HashMap<TypeId, UntypedHandle>,
    #[cfg(feature = "progress_tracking")]
    progress: Progress,
}

impl ManifestLoader {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "progress_tracking")]
    fn inc_progress_todo(&mut self) {
        self.progress.total += 2;
    }

    #[cfg(feature = "progress_tracking")]
    fn inc_progress_done(&mut self) {
        self.progress.done += 1;
    }

    #[cfg(feature = "progress_tracking")]
    pub fn progress(&self) -> &Progress {
        &self.progress
    }

    pub fn register<M: Manifest>(&mut self, path: impl Into<PathBuf>) {
        let path: PathBuf = path.into();
        let type_id = std::any::TypeId::of::<ManifestCollection<M>>();

        self.to_load.insert(type_id, path);

        #[cfg(feature = "progress_tracking")]
        self.inc_progress_todo();
    }

    pub fn load<M: Manifest>(&mut self, asset_server: &mut AssetServer) {
        let type_id = std::any::TypeId::of::<ManifestCollection<M>>();
        let Some(path) = self.to_load.get(&type_id) else {
            error!("Load called without path: {:?}", type_name::<M>());
            return;
        };

        let handle = asset_server
            .load::<ManifestCollection<M>>(path.clone())
            .untyped();

        self.loaded.insert(type_id, handle);
        self.to_load.remove(&type_id);

        #[cfg(feature = "progress_tracking")]
        self.inc_progress_done();
    }

    pub fn is_loaded<M: Manifest>(&self) -> bool {
        let type_id = std::any::TypeId::of::<ManifestCollection<M>>();
        self.loaded.contains_key(&type_id)
    }

    pub fn process<M: Manifest<Output = P>, P: Prototype>(
        &mut self,
        assets: &mut Assets<ManifestCollection<M>>,
        protos: &mut PrototypeLibrary<P>,
    ) {
        let id = std::any::TypeId::of::<ManifestCollection<M>>();
        let untyped_handle = self.loaded.get(&id).unwrap();
        let typed_handle = untyped_handle.clone_weak().typed::<ManifestCollection<M>>();

        let Some(manifest) = assets.get(&typed_handle) else {
            error!("Failed to get manifest: {:?}", id);
            return;
        };

        protos.clear();
        for m in manifest {
            let item: P = m.reify();
            protos.insert(item);
        }

        self.processed.insert(id, untyped_handle.clone());

        #[cfg(feature = "progress_tracking")]
        self.inc_progress_done();
    }
}
