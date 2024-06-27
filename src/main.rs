use bevy::{ecs::system::EntityCommand, log::LogPlugin, prelude::*};
use rantz_proto::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "error,rantz_proto=debug".into(),
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(ProtoPlugin)
        .add_prototypes::<TestManifest, TestProto>("items.ron")
        .add_systems(Update, (spawn_stuff, check_stuff, count_stuff))
        .run();
}

#[derive(Debug, Clone)]
pub struct TestProto {
    pub name: String,
}

impl Prototype for TestProto {}

impl Display for TestProto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestProto({})", self.name)
    }
}

impl EntityCommand for TestProto {
    fn apply(self, entity: Entity, world: &mut World) {
        let mut target = world.entity_mut(entity);
        target.insert(Name::new(self.name));
    }
}

#[derive(Serialize, Deserialize, Asset, TypePath, Clone)]
pub struct TestManifest {
    pub name: String,
}

impl Manifest for TestManifest {
    const FORMAT: ManifestFormat = ManifestFormat::Ron;
    type Output = TestProto;
    fn reify(&self) -> TestProto {
        TestProto {
            name: self.name.clone(),
        }
    }
}

fn check_stuff(protos: Res<PrototypeLibrary<TestProto>>) {
    if !protos.is_empty() {
        debug!("All manifests loaded");
    }

    debug!("{:#?}", protos);
}

fn spawn_stuff(
    mut commands: Commands,
    protos: Res<PrototypeLibrary<TestProto>>,
    mut counter: Local<u32>,
) {
    if protos.is_empty() {
        return;
    }

    if *counter < 10 {
        *counter += 1;
        commands.spawn_prototype(protos.first().unwrap());
    }
}

fn count_stuff(query: Query<&Name>) {
    let count = query.iter().count();

    debug!("I see {} entities", count);
}
