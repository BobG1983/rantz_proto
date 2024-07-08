use crate::prelude::*;
use bevy::{
    ecs::world::CommandQueue,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};

#[derive(Component, Debug)]
pub(crate) struct ProtoSpawnTask(pub Task<CommandQueue>);

/// An extension trait for [`Commands`] [`CommandQueue`] and [`World`]
/// to allow direct spawning of prototypes
pub trait SpawnPrototypeExt {
    /// Spawns the given prototype
    fn spawn_prototype<P: Prototype>(&mut self, proto: P);
}

/// An extension trait for [`Commands`] [`CommandQueue`] and [`World`]
/// to allow direct spawning of prototypes in an async task
pub trait SpawnPrototypeAsyncExt {
    /// Spawns the given prototype in an async task
    fn spawn_prototype_async<P: Prototype>(&mut self, proto: P);
}

impl<'w, 's> SpawnPrototypeExt for Commands<'w, 's> {
    fn spawn_prototype<P: Prototype>(&mut self, proto: P) {
        let mut e = self.spawn_empty();

        #[cfg(feature = "hot_reload")]
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
            proto.build(&mut target);

            #[cfg(feature = "hot_reload")]
            target.insert(FromPrototype(proto.id()));
        });
    }
}

impl SpawnPrototypeExt for World {
    fn spawn_prototype<P: Prototype>(&mut self, proto: P) {
        let mut target = self.spawn_empty();
        proto.build(&mut target);

        #[cfg(feature = "hot_reload")]
        target.insert(FromPrototype(proto.id()));
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
