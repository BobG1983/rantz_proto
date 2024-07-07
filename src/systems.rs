use crate::prelude::*;
use bevy::{
    prelude::*,
    tasks::{block_on, poll_once},
};
#[cfg(feature = "progress_tracking")]
use iyes_progress::Progress;

pub fn load<M: Manifest>(
    mut asset_server: ResMut<AssetServer>,
    mut loader: ResMut<ManifestLoader>,
) {
    if loader.is_loaded::<M>() {
        return;
    }

    loader.load::<M>(&mut asset_server);
}

#[cfg(feature = "hot_reload")]
pub fn track_asset<M: Manifest<Output = P>, P: Prototype>(
    mut commands: Commands,
    events: EventReader<AssetEvent<ManifestCollection<M>>>,
    assets: ResMut<Assets<ManifestCollection<M>>>,
    protos: ResMut<PrototypeLibrary<P>>,
    loader: ResMut<ManifestLoader>,
    rebuild_systems: Res<RebuildSystems>,
) {
    use std::any::{type_name, TypeId};

    if handle_asset_events(events, loader, assets, protos) {
        let type_id = TypeId::of::<P>();
        let Some(system_id) = rebuild_systems.0.get(&type_id) else {
            error!("Rebuild system not registered for: {:?}", type_name::<P>());
            return;
        };
        commands.run_system(*system_id);
    }
}

#[cfg(not(feature = "hot_reload"))]
pub fn track_asset<M: Manifest<Output = P>, P: Prototype>(
    events: EventReader<AssetEvent<ManifestCollection<M>>>,
    assets: ResMut<Assets<ManifestCollection<M>>>,
    protos: ResMut<PrototypeLibrary<P>>,
    loader: ResMut<ManifestLoader>,
) {
    handle_asset_events(events, loader, assets, protos);
}

fn handle_asset_events<M: Manifest<Output = P>, P: Prototype>(
    mut events: EventReader<AssetEvent<ManifestCollection<M>>>,
    mut loader: ResMut<ManifestLoader>,
    mut assets: ResMut<Assets<ManifestCollection<M>>>,
    mut protos: ResMut<PrototypeLibrary<P>>,
) -> bool {
    let mut rebuilt = false;
    for ev in events.read() {
        if let AssetEvent::LoadedWithDependencies { id: _ } = ev {
            loader.process::<M, P>(&mut assets, &mut protos);

            rebuilt = true;
        }
    }

    rebuilt
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

#[cfg(feature = "progress_tracking")]
pub fn track_progress(loader: Res<ManifestLoader>) -> Progress {
    *loader.progress()
}
