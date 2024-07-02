use bevy::prelude::EntityWorldMut;
use std::fmt::Debug;

use crate::id::Id;

/// A trait for building prototypes
pub trait Prototype: Clone + Send + Sync + Debug + 'static {
    /// The name of the prototype, must be unique within as
    /// this is used to identify the prototype in the PrototypeLibrary
    fn name(&self) -> String;

    /// Default id of the prototype uses the name with Id::from_name
    fn id(&self) -> Id<Self> {
        Id::from_name(&self.name())
    }

    /// Builds the prototype on a target entity
    fn build(&self, target: &mut EntityWorldMut);

    /// Rebuilds the prototype on a target entity if 'hot_reload' is enabled
    /// by default this does nothing
    fn rebuild(&self, _: &mut EntityWorldMut) {}
}
