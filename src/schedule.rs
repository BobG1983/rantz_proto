use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ProtoSchedule {
    Processing,
    Loading,
}
