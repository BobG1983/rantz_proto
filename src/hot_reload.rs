use crate::prelude::*;
use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};
use std::any::TypeId;

#[derive(Resource)]
pub(crate) struct RebuildSystems(pub HashMap<TypeId, SystemId>);

impl RebuildSystems {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for RebuildSystems {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

fn rebuild<P: Prototype>(
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

pub fn register_rebuild_system<P: Prototype>(world: &mut World) {
    let type_id = TypeId::of::<P>();
    let system_id = world.register_system(rebuild::<P>);

    check_insert_rebuild_system(world);
    let mut res = world.resource_mut::<RebuildSystems>();

    res.0.insert(type_id, system_id);
}

fn check_insert_rebuild_system(world: &mut World) {
    if world.get_resource::<RebuildSystems>().is_none() {
        let res = RebuildSystems::new();
        world.insert_resource(res);
    }
}
