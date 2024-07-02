use crate::prelude::*;
use bevy::{
    ecs::entity,
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
    mut events: EventReader<AssetEvent<ManifestCollection<M>>>,
    mut assets: ResMut<Assets<ManifestCollection<M>>>,
    mut protos: ResMut<PrototypeLibrary<P>>,
    mut loader: ResMut<ManifestLoader>,
) {
    for ev in events.read() {
        if let AssetEvent::LoadedWithDependencies { id: _ } = ev {
            loader.process::<M, P>(&mut assets, &mut protos)
        }
    }
}

pub fn rebuild<P: Prototype>(
    mut commands: Commands,
    query: Query<(Entity, &FromPrototype<P>)>,
    protos: ResMut<PrototypeLibrary<P>>,
) {
    for (e, proto) in query.iter() {
        if let Some(proto) = protos.get(&proto.0) {
            let mut target = commands.entity(e);
            target.add(move |mut e: EntityWorldMut| proto.build(&mut e));
        }
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
