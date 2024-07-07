use crate::{prelude::*, systems::track_progress};
use bevy::prelude::*;
use iyes_progress::{ProgressSystem, TrackedProgressSet};

pub struct ProtoPlugin;

impl Plugin for ProtoPlugin {
    fn build(&self, app: &mut App) {
        Self::init_resources(app);
        Self::add_systems(app);
    }
}

impl ProtoPlugin {
    pub fn init_resources(app: &mut App) {
        app.init_resource::<ManifestLoader>();
    }

    pub fn add_systems(app: &mut App) {
        app.add_systems(
            PostUpdate,
            handle_async_spawn.in_set(ProtoSchedule::Spawning),
        );
        #[cfg(feature = "progress_tracking")]
        app.add_systems(
            Update,
            track_progress
                .track_progress()
                .in_set(TrackedProgressSet)
                .in_set(ProtoSchedule::Loading),
        );
    }
}
