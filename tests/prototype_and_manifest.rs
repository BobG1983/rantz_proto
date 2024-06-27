mod common;
use bevy::{
    ecs::system::{CommandQueue, EntityCommand},
    prelude::*,
};
use common::*;
use rantz_proto::prelude::*;

#[test]
fn reify_manifest_to_prototype() {
    let manifest = TestManifest { i: 0 };
    let proto = manifest.reify();
    assert_eq!(proto.i, 0);
}

#[test]
fn apply_prototype_world() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let proto = TestProto { i: 0 };
    proto.apply(e, &mut world);

    let counter = world.entity(e).get::<Counter>().unwrap();
    assert_eq!(counter.i, 0);
}

#[test]
fn apply_prototype_with_commands() {
    let mut world = World::new();
    let mut command_queue = CommandQueue::default();

    let proto = TestProto { i: 0 };
    let e = Commands::new(&mut command_queue, &world)
        .spawn_empty()
        .add(proto)
        .id();

    command_queue.apply(&mut world);

    let counter = world.entity(e).get::<Counter>().unwrap();
    assert_eq!(counter.i, 0);
}
