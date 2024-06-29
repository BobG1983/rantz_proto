use bevy::{app::AppExit, ecs::system::EntityCommand, log::LogPlugin, prelude::*};
use rantz_proto::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "error,rantz_proto=debug".into(),
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(ProtoPlugin)
        .add_prototype_collection::<TestManifest, TestProto>("items.ron")
        .add_systems(Update, transition_on_esc)
        .add_systems(
            Update,
            (spawn_stuff, check_stuff, count_stuff).run_if(in_state(GameState::Running)),
        )
        .configure_sets(
            Update,
            ProtoSchedule::Loading.run_if(in_state(GameState::Running)),
        )
        .run();
}

#[derive(Debug, Clone, States, Hash, PartialEq, Eq, Default)]
enum GameState {
    #[default]
    Loading,
    Running,
}

#[derive(Debug, Clone, Prototype)]
struct TestProto {
    pub name: String,
    #[expect(dead_code)]
    pub test: TestEnum,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
enum TestEnum {
    A,
    B,
}

#[derive(Serialize, Deserialize, Asset, TypePath, Clone)]
struct TestManifest {
    pub name: String,
    pub test: TestEnum,
}

impl Manifest for TestManifest {
    const FORMAT: ManifestFormat = ManifestFormat::Ron;
    type Output = TestProto;
    fn reify(&self) -> TestProto {
        TestProto {
            name: self.name.clone(),
            test: self.test.clone(),
        }
    }
}

fn transition_on_esc(mut state: ResMut<NextState<GameState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        debug!("Transitioning to Running");
        state.set(GameState::Running);
    }
}

fn check_stuff(protos: Res<PrototypeLibrary<TestProto>>, mut done: Local<bool>) {
    if !*done && !protos.is_empty() {
        debug!("All manifests loaded");
        *done = true;
    }
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
        commands.spawn_prototype_async(protos.first().unwrap());
    }
}

fn count_stuff(query: Query<&Name>, mut events: EventWriter<AppExit>) {
    let count = query.iter().count();

    if count <= 10 {
        debug!("I see {} entities", count)
    };

    if count >= 10 {
        debug!("Exiting");
        events.send_default();
    }
}
