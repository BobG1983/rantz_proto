use crate::prelude::*;
use bevy::{ecs::system::CommandQueue, prelude::*};

pub trait SpawnProtoTypeExt {
    fn spawn_prototype(&mut self, proto: impl Prototype);
}

impl<'w, 's> SpawnProtoTypeExt for Commands<'w, 's> {
    fn spawn_prototype(&mut self, proto: impl Prototype) {
        let mut e = self.spawn_empty();
        e.add(proto);
    }
}

impl SpawnProtoTypeExt for CommandQueue {
    fn spawn_prototype(&mut self, proto: impl Prototype) {
        self.push(move |world: &mut World| {
            let id = world.spawn_empty().id();
            proto.apply(id, world);
        });
    }
}
