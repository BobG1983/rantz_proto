use bevy::ecs::system::EntityCommand;
use std::fmt::{Debug, Display};

pub trait Prototype: EntityCommand + Clone + Send + Sync + Display + Debug + 'static {}
