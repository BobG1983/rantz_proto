use crate::prelude::*;
use bevy::{
    ecs::system::CommandQueue,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};

#[derive(Component, Debug)]
pub(crate) struct ProtoSpawnTask(pub Task<CommandQueue>);

pub trait SpawnPrototypeExt {
    fn spawn_prototype(&mut self, proto: impl Prototype);
}

pub trait SpawnPrototypeAsyncExt {
    fn spawn_prototype_async(&mut self, proto: impl Prototype);
}

impl<'w, 's> SpawnPrototypeExt for Commands<'w, 's> {
    fn spawn_prototype(&mut self, proto: impl Prototype) {
        let mut e = self.spawn_empty();
        e.add(proto);
    }
}
impl<'w, 's> SpawnPrototypeAsyncExt for Commands<'w, 's> {
    fn spawn_prototype_async(&mut self, proto: impl Prototype) {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            let mut queue = CommandQueue::default();
            queue.spawn_prototype(proto);
            queue
        });

        self.spawn_empty()
            .insert(ProtoSpawnTask(task))
            .insert(Name::new("Prototype Loading"));
    }
}

impl SpawnPrototypeExt for CommandQueue {
    fn spawn_prototype(&mut self, proto: impl Prototype) {
        self.push(move |world: &mut World| {
            let id = world.spawn_empty().id();
            proto.apply(id, world);
        });
    }
}

impl SpawnPrototypeExt for World {
    fn spawn_prototype(&mut self, proto: impl Prototype) {
        let id = self.spawn_empty().id();
        proto.apply(id, self);
    }
}
impl SpawnPrototypeAsyncExt for World {
    fn spawn_prototype_async(&mut self, proto: impl Prototype) {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            let mut queue = CommandQueue::default();
            queue.spawn_prototype(proto);
            queue
        });

        self.spawn_empty()
            .insert(ProtoSpawnTask(task))
            .insert(Name::new("Prototype Loading"));
    }
}
