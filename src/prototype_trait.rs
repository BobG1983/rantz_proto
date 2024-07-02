use bevy::prelude::{Component, EntityWorldMut};
use std::fmt::Debug;

use crate::id::Id;

pub trait Prototype: Clone + Send + Sync + Debug + 'static {
    fn name(&self) -> String;
    fn id(&self) -> Id<Self> {
        Id::from_name(&self.name())
    }
    fn build(&self, target: &mut EntityWorldMut);
    fn rebuild(&self, _: &mut EntityWorldMut) {}
}

#[derive(Debug, Clone, Component, PartialEq, Eq)]
pub struct FromPrototype<P: Prototype>(pub Id<P>);
