use bevy::prelude::EntityWorldMut;
use std::fmt::{Debug, Display};

pub trait Prototype: Clone + Send + Sync + Display + Debug + 'static {
    fn build(&self, target: &mut EntityWorldMut);
    fn rebuild(&self, _: &mut EntityWorldMut) {}
}
