use std::any::{type_name, TypeId};

use crate::prelude::*;
use bevy::{
    prelude::*,
    tasks::{block_on, poll_once},
};

pub fn load<M: Manifest>(
    mut asset_server: ResMut<AssetServer>,
    mut loader: ResMut<ManifestLoader>,
) {
    if loader.is_loaded::<M>() {
        return;
    }

    loader.load::<M>(&mut asset_server);
}

pub fn track_asset<M: Manifest<Output = P>, P: Prototype>(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<ManifestCollection<M>>>,
    mut assets: ResMut<Assets<ManifestCollection<M>>>,
    mut protos: ResMut<PrototypeLibrary<P>>,
    mut loader: ResMut<ManifestLoader>,
    rebuild_systems: Option<Res<RebuildSystems>>,
) {
    let mut need_rebuild = false;
    for ev in events.read() {
        if let AssetEvent::LoadedWithDependencies { id: _ } = ev {
            loader.process::<M, P>(&mut assets, &mut protos);

            need_rebuild = true;
        }
    }

    if need_rebuild {
        let Some(rebuild_systems) = rebuild_systems else {
            error!("RebuildSystems resource not registered");
            return;
        };

        let type_id = TypeId::of::<P>();
        let Some(system_id) = rebuild_systems.0.get(&type_id) else {
            error!("Rebuild system not registered for: {:?}", type_name::<P>());
            return;
        };
        commands.run_system(*system_id);
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
