use bevy_ecs::system::EntityCommand;
use std::fmt::{Debug, Display};

pub trait Prototype: EntityCommand + Sync + Display + Debug {}
