use crate::prelude::*;
use bevy::prelude::*;

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
        app.add_systems(Update, update_loader.in_set(ProtoSchedule::Loading))
            .add_systems(
                PostUpdate,
                handle_async_spawn.in_set(ProtoSchedule::Spawning),
            );
    }
}
