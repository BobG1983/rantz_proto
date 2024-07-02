use crate::prelude::*;
use bevy::{
    prelude::*,
    tasks::{block_on, poll_once},
};

pub fn update_loader_status(asset_server: Res<AssetServer>, mut loader: ResMut<ManifestLoader>) {
    loader.update_load_states(&asset_server);
}

pub fn load<M: Manifest>(
    mut asset_server: ResMut<AssetServer>,
    mut loader: ResMut<ManifestLoader>,
    mut has_loaded: Local<bool>,
) {
    if *has_loaded {
        return;
    }

    loader.load::<M>(&mut asset_server);
    *has_loaded = true;
}

pub fn process<M: Manifest<Output = P>, P: Prototype>(
    mut loader: ResMut<ManifestLoader>,
    mut assets: ResMut<Assets<ManifestCollection<M>>>,
    mut protos: ResMut<PrototypeLibrary<P>>,
) {
    if loader.is_empty() {
        return;
    }

    loader.process::<M, P>(&mut assets, &mut protos);
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
