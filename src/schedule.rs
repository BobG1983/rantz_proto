use bevy::prelude::*;

/// The schedule for loading and spawning prototypes
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ProtoSchedule {
    /// The schedule for loading prototypes. If you want to control when
    /// prototypes are loaded, you can `configure_sets` on this schedule
    Loading,
    /// The schedule for spawning prototypes whose spawn was requested
    /// asynchronously
    Spawning,
}
