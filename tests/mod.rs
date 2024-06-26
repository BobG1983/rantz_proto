#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use bevy_asset::Asset;
    use bevy_ecs::{
        prelude::*,
        system::{CommandQueue, EntityCommand},
    };
    use bevy_reflect::TypePath;
    use rantz_proto::prelude::*;
    use serde::Deserialize;

    #[derive(Component)]
    struct Counter {
        i: i32,
    }

    #[derive(Debug)]
    struct TestProto {
        i: i32,
    }

    impl Prototype for TestProto {}

    impl Display for TestProto {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestProto({})", self.i)
        }
    }

    impl EntityCommand for TestProto {
        fn apply(self, entity: Entity, world: &mut World) {
            let mut target = world.entity_mut(entity);
            target.insert(Counter { i: self.i });
        }
    }

    #[derive(Deserialize, Asset, TypePath)]
    struct TestManifest {
        i: usize,
    }

    impl Manifest<TestProto> for TestManifest {
        fn reify(&self) -> TestProto {
            TestProto { i: self.i as i32 }
        }
    }

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

    #[test]
    fn prototype_library_should_start_empty() {
        let library = PrototypeLibrary::<TestProto>::new();
        assert!(library.is_empty());
    }

    #[test]
    fn can_insert_prototype_into_library() {
        let mut library = PrototypeLibrary::<TestProto>::new();

        let proto = TestProto { i: 0 };
        let id = library.insert(proto);

        assert!(id.is_ok());
    }

    #[test]
    fn can_get_prototype_from_library() {
        let mut library = PrototypeLibrary::<TestProto>::new();

        let proto = TestProto { i: 0 };
        let id = library.insert(proto);

        assert!(id.is_ok());

        let id = id.unwrap();
        let proto = library.get(id);
        assert!(proto.is_some());
        assert_eq!(proto.unwrap().i, 0);
    }

    #[test]
    fn can_get_prototype_from_library_by_name() {
        let mut library = PrototypeLibrary::<TestProto>::new();
        let proto = TestProto { i: 0 };
        let name = proto.to_string();
        let _ = library.insert(proto);
        let proto = library.get_by_name(&name);
        assert!(proto.is_some());
        assert_eq!(proto.unwrap().i, 0);
    }

    #[test]
    fn can_mutate_prototype_in_library() {
        let mut library = PrototypeLibrary::<TestProto>::new();

        let proto = TestProto { i: 0 };
        let id = library.insert(proto);

        assert!(id.is_ok());

        let id = id.unwrap();
        let proto = library.get_mut(id);
        assert!(proto.is_some());

        let proto = proto.unwrap();
        proto.i = 1;

        let proto = library.get(id);
        assert!(proto.is_some());
        assert_eq!(proto.unwrap().i, 1);
    }

    #[test]
    fn can_mutate_prototype_in_library_by_name() {
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
    fn can_remove_prototype_from_library() {
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
    fn can_remove_prototype_from_library_by_name() {
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
    fn can_access_library_as_resource() {
        let mut library = PrototypeLibrary::<TestProto>::new();
        assert!(library.is_empty());

        let proto = TestProto { i: 0 };
        let _ = library.insert(proto);
        assert!(!library.is_empty());

        library.clear();
        assert!(library.is_empty());
    }
}
