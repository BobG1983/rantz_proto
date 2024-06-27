use crate::prelude::*;
use bevy::prelude::*;

pub struct ProtoPlugin;

impl Plugin for ProtoPlugin {
    fn build(&self, app: &mut App) {
        Self::init_resources(app);
        Self::add_systems(app);
        Self::configure_sets(app);
    }
}

impl ProtoPlugin {
    pub fn init_resources(app: &mut App) {
        app.init_resource::<ManifestLoader>();
    }

    pub fn add_systems(app: &mut App) {
        app.add_systems(
            PreUpdate,
            update_manifest_loader.in_set(ProtoSchedule::Loading),
        );
    }

    pub fn configure_sets(app: &mut App) {
        app.configure_sets(
            PreUpdate,
            (ProtoSchedule::Loading, ProtoSchedule::Processing).chain(),
        );
    }
}
