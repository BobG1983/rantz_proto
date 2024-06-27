use bevy::{ecs::system::EntityCommand, prelude::*};
use rantz_proto::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Component)]
pub struct Counter {
    pub i: i32,
}

#[derive(Debug, Clone)]
pub struct TestProto {
    pub i: i32,
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

#[derive(Serialize, Deserialize, Asset, TypePath, Clone)]
pub struct TestManifest {
    pub i: usize,
}

impl Manifest for TestManifest {
    const FORMAT: ManifestFormat = ManifestFormat::Ron;
    type Output = TestProto;
    fn reify(&self) -> TestProto {
        TestProto { i: self.i as i32 }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EmptyProto;
impl Prototype for EmptyProto {}

impl Display for EmptyProto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmptyProto")
    }
}

impl EntityCommand for EmptyProto {
    fn apply(self, _: Entity, _: &mut World) {}
}

#[derive(Serialize, Deserialize, Asset, TypePath, Clone)]
pub struct EmptyManifest;

impl Manifest for EmptyManifest {
    const FORMAT: ManifestFormat = ManifestFormat::Ron;
    type Output = EmptyProto;
    fn reify(&self) -> EmptyProto {
        EmptyProto
    }
}
