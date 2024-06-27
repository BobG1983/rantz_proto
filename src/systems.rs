use crate::prelude::*;
use bevy::{
    asset::LoadState,
    prelude::*,
    tasks::{block_on, poll_once},
};
use std::any::type_name;

pub fn update_manifest_loader(asset_server: Res<AssetServer>, mut loader: ResMut<ManifestLoader>) {
    loader.update_load_states(&asset_server);
}

pub fn process_manifest<M: Manifest<Output = P>, P: Prototype>(
    loader: Res<ManifestLoader>,
    mut assets: ResMut<Assets<M>>,
    mut protos: ResMut<PrototypeLibrary<P>>,
) {
    if !assets.is_changed() {
        return;
    }

    let Some(status) = loader.get_manifest_status::<M>() else {
        error!(
            "Manifest not found in manifest loader: {:?}",
            type_name::<M>()
        );
        return;
    };

    match status.load_state {
        LoadState::Loaded => {
            let handle = status.handle.clone_weak().typed::<M>();
            if let Some(manifest) = assets.remove(handle) {
                let item: P = manifest.reify();

                if let Err(e) = protos.insert(item) {
                    error!("{}", e)
                }
            }
        }
        LoadState::Failed => {
            error!("Manifest failed to load: {:?}", type_name::<M>());
        }
        _ => {}
    }
}

pub fn process_manifest_collection<M: Manifest<Output = P>, P: Prototype>(
    loader: Res<ManifestLoader>,
    mut assets: ResMut<Assets<ManifestCollection<M>>>,
    mut protos: ResMut<PrototypeLibrary<P>>,
) {
    if !assets.is_changed() {
        return;
    }

    let Some(status) = loader.get_manifest_status::<M>() else {
        error!(
            "Manifest not found in manifest loader: {:?}",
            type_name::<M>()
        );
        return;
    };

    match status.load_state {
        LoadState::Loaded => {
            let handle = status.handle.clone_weak().typed::<ManifestCollection<M>>();
            if let Some(collection) = assets.remove(handle) {
                for manifest in collection {
                    let item: P = manifest.reify();

                    if let Err(e) = protos.insert(item) {
                        error!("{}", e)
                    }
                }
            }
        }
        LoadState::Failed => {
            error!("Manifest failed to load: {:?}", type_name::<M>());
        }
        _ => {}
    }
}

pub fn handle_async_spawn(
    mut commands: Commands,
    mut load_tasks: Query<(Entity, &mut ProtoSpawnTask)>,
) {
    for (e, mut load_task) in load_tasks.iter_mut() {
        if load_task.0.is_finished() {
            if let Some(mut command_queue) = block_on(poll_once(&mut load_task.0)) {
                commands.append(&mut command_queue);
                commands.entity(e).despawn_recursive();
            }
        }
    }
}
