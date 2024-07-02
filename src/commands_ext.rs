use crate::prelude::*;
use bevy::{
    ecs::system::CommandQueue,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};

#[derive(Component, Debug)]
pub(crate) struct ProtoSpawnTask(pub Task<CommandQueue>);

pub trait SpawnPrototypeExt {
    fn spawn_prototype<P: Prototype>(&mut self, proto: P);
}

pub trait SpawnPrototypeAsyncExt {
    fn spawn_prototype_async<P: Prototype>(&mut self, proto: P);
}

impl<'w, 's> SpawnPrototypeExt for Commands<'w, 's> {
    fn spawn_prototype<P: Prototype>(&mut self, proto: P) {
        let mut e = self.spawn_empty();
        e.insert(FromPrototype(proto.id()));
        e.add(move |mut e: EntityWorldMut| proto.build(&mut e));
    }
}
impl<'w, 's> SpawnPrototypeAsyncExt for Commands<'w, 's> {
    fn spawn_prototype_async<P: Prototype>(&mut self, proto: P) {
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
    fn spawn_prototype<P: Prototype>(&mut self, proto: P) {
        self.push(move |world: &mut World| {
            let mut target = world.spawn_empty();
            target.insert(FromPrototype(proto.id()));
            proto.build(&mut target);
        });
    }
}

impl SpawnPrototypeExt for World {
    fn spawn_prototype<P: Prototype>(&mut self, proto: P) {
        let mut target = self.spawn_empty();
        target.insert(FromPrototype(proto.id()));
        proto.build(&mut target);
    }
}
impl SpawnPrototypeAsyncExt for World {
    fn spawn_prototype_async<P: Prototype>(&mut self, proto: P) {
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
