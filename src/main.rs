use bevy::{app::AppExit, log::LogPlugin, prelude::*};
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
        .add_prototype::<TestManifest, TestProto>("items.ron")
        .add_systems(
            Update,
            transition_on_esc.run_if(in_state(GameState::Waiting)),
        )
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
    Waiting,
    Running,
}

#[derive(Debug, Clone)]
struct TestProto {
    pub name: String,
}

impl Display for TestProto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestProto({})", self.name)
    }
}

impl Prototype for TestProto {
    fn build(&self, target: &mut EntityWorldMut) {
        target.insert(Name::new(self.name.clone()));
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Asset, TypePath, Clone)]
struct TestManifest {
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

fn transition_on_esc(mut state: ResMut<NextState<GameState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        debug!("Transitioning to Running");
        state.set(GameState::Running);
    }
}

fn check_stuff(protos: Res<PrototypeLibrary<TestProto>>, mut done: Local<bool>) {
    if !*done && !protos.is_empty() {
        debug!("{:?} prototypes loaded", protos.len());
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

    if *counter < 1 {
        *counter += 1;
        commands.spawn_prototype_async(protos.first().unwrap());
    }
}

fn count_stuff(
    query: Query<(Entity, &Name)>,
    mut events: EventWriter<AppExit>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (entity, name) in query.iter() {
        debug!("{:?}: {:?}", name, entity);
    }

    if input.just_pressed(KeyCode::Escape) {
        debug!("Exiting");
        events.send_default();
    }
}
