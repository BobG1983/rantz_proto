use bevy::prelude::EntityWorldMut;
use std::fmt::Debug;

pub trait Prototype: Clone + Send + Sync + Debug + 'static {
    fn name(&self) -> String;
    fn build(&self, target: &mut EntityWorldMut);
    fn rebuild(&self, _: &mut EntityWorldMut) {}
}
