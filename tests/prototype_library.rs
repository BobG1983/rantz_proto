use bevy::prelude::*;
use rantz_proto::prelude::*;
mod common;
use common::*;

#[test]
fn prototype_library_should_start_empty() {
    let library = PrototypeLibrary::<TestProto>::new();
    assert!(library.is_empty());
}

#[test]
fn insert_prototype_into_library() {
    let mut library = PrototypeLibrary::<TestProto>::new();

    let proto = TestProto { i: 0 };
    let id = library.insert(proto);

    assert!(id.is_ok());
}

#[test]
fn insert_prototype_into_library_from_manifest() {
    let mut library = PrototypeLibrary::<TestProto>::new();
    let manifest = TestManifest { i: 0 };

    let id = library.insert_from_manifest(manifest);
    assert!(id.is_ok());

    let proto = library.get(&id.unwrap());
    assert!(proto.is_some());
    assert_eq!(proto.unwrap().i, 0);
}

#[test]
fn get_prototype_from_library() {
    let mut library = PrototypeLibrary::<TestProto>::new();

    let proto = TestProto { i: 0 };
    let id = library.insert(proto);

    assert!(id.is_ok());

    let id = id.unwrap();
    let proto = library.get(&id);
    assert!(proto.is_some());
    assert_eq!(proto.unwrap().i, 0);
}

#[test]
fn get_prototype_from_library_by_name() {
    let mut library = PrototypeLibrary::<TestProto>::new();
    let proto = TestProto { i: 0 };
    let name = proto.to_string();
    let _ = library.insert(proto);
    let proto = library.get_by_name(&name);
    assert!(proto.is_some());
    assert_eq!(proto.unwrap().i, 0);
}

#[test]
fn mutate_prototype_in_library() {
    let mut library = PrototypeLibrary::<TestProto>::new();

    let proto = TestProto { i: 0 };
    let id = library.insert(proto);

    assert!(id.is_ok());

    let id = id.unwrap();
    let proto = library.get_mut(&id);
    assert!(proto.is_some());

    let proto = proto.unwrap();
    proto.i = 1;

    let proto = library.get(&id);
    assert!(proto.is_some());
    assert_eq!(proto.unwrap().i, 1);
}

#[test]
fn mutate_prototype_in_library_by_name() {
    let mut library = PrototypeLibrary::<TestProto>::new();

    let proto = TestProto { i: 0 };
    let name = proto.to_string();
    let _ = library.insert(proto);
    let proto = library.get_mut_by_name(&name);
    assert!(proto.is_some());

    let proto = proto.unwrap();
    proto.i = 1;

    let proto = library.get_mut_by_name(&name);
    assert!(proto.is_some());
    assert_eq!(proto.unwrap().i, 1);
}

#[test]
fn remove_prototype_from_library() {
    let mut library = PrototypeLibrary::<TestProto>::new();

    let proto = TestProto { i: 0 };
    let id = library.insert(proto);

    assert!(id.is_ok());

    let id = id.unwrap();
    let proto = library.remove(id);
    assert!(proto.is_ok());
    assert_eq!(proto.unwrap().i, 0);
    assert!(library.is_empty());
}

#[test]
fn remove_prototype_from_library_by_name() {
    let mut library = PrototypeLibrary::<TestProto>::new();
    let proto = TestProto { i: 0 };
    let name = proto.to_string();
    let _ = library.insert(proto);
    let proto = library.remove_by_name(&name);
    assert!(proto.is_ok());
    assert_eq!(proto.unwrap().i, 0);
    assert!(library.is_empty());
}

#[test]
fn clear_prototype_library() {
    let mut library = PrototypeLibrary::<TestProto>::new();
    assert!(library.is_empty());

    let proto = TestProto { i: 0 };
    let _ = library.insert(proto);
    assert!(!library.is_empty());

    library.clear();
    assert!(library.is_empty());
}

#[test]
fn access_library_as_res() {
    let mut world = World::new();
    world.init_resource::<PrototypeLibrary<TestProto>>();
    let mut schedule = Schedule::default();
    schedule.add_systems(|protos: Res<PrototypeLibrary<TestProto>>| {
        assert!(protos.is_empty());
    });

    schedule.run(&mut world);
}

#[test]
fn access_library_as_resmut() {
    let mut world = World::new();
    world.init_resource::<PrototypeLibrary<TestProto>>();
    let mut schedule = Schedule::default();
    schedule.add_systems(|mut protos: ResMut<PrototypeLibrary<TestProto>>| {
        assert!(protos.is_empty());
        let id = protos.insert_from_manifest(TestManifest { i: 0 });
        assert!(!protos.is_empty());
        assert!(id.is_ok());
        let proto = protos.get(&id.unwrap());
        assert!(proto.is_some());
        assert_eq!(proto.unwrap().i, 0);
    });

    schedule.run(&mut world);
}

#[test]
fn access_multiple_library_as_res() {
    let mut world = World::new();
    world.init_resource::<PrototypeLibrary<TestProto>>();
    world.init_resource::<PrototypeLibrary<EmptyProto>>();
    let mut schedule = Schedule::default();
    schedule.add_systems(
        |test_protos: Res<PrototypeLibrary<TestProto>>,
         empty_protos: Res<PrototypeLibrary<EmptyProto>>| {
            assert!(test_protos.is_empty());
            assert!(empty_protos.is_empty());
        },
    );

    schedule.run(&mut world);
}

#[test]
fn access_multiple_library_as_resmut() {
    let mut world = World::new();
    world.init_resource::<PrototypeLibrary<TestProto>>();
    world.init_resource::<PrototypeLibrary<EmptyProto>>();
    let mut schedule = Schedule::default();
    schedule.add_systems(
        |mut test_protos: ResMut<PrototypeLibrary<TestProto>>,
         mut empty_protos: ResMut<PrototypeLibrary<EmptyProto>>| {
            let test_id = test_protos
                .insert_from_manifest(TestManifest { i: 0 })
                .unwrap();
            let test_proto = test_protos.get(&test_id).unwrap();
            assert_eq!(test_proto.i, 0);

            let empty_id = empty_protos.insert_from_manifest(EmptyManifest).unwrap();
            let empty_proto = empty_protos.get(&empty_id).unwrap();
            assert_eq!(empty_proto, EmptyProto);
        },
    );

    schedule.run(&mut world);
}
