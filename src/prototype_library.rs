use crate::prelude::*;
use bevy_ecs::system::Resource;
use bevy_utils::HashMap;
use thiserror::Error;

#[derive(Default, Debug, Resource)]
pub struct PrototypeLibrary<T>
where
    T: Prototype,
{
    prototypes: HashMap<Id<T>, T>,
}

impl<T> PrototypeLibrary<T>
where
    T: Prototype,
{
    pub fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    #[must_use]
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.prototypes.get(&id)
    }

    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        self.get(Id::from_name(name))
    }

    #[must_use]
    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        self.prototypes.get_mut(&id)
    }

    #[must_use]
    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut T> {
        self.get_mut(Id::from_name(name))
    }

    pub fn insert(&mut self, item: T) -> Result<Id<T>, LibraryError> {
        let id = Id::from_name(&item.to_string());
        if self.prototypes.contains_key(&id) {
            Err(LibraryError::DuplicateName(item.to_string()))
        } else {
            self.prototypes.insert(id, item);
            Ok(id)
        }
    }
    pub fn insert_from_manifest(
        &mut self,
        manifest: impl Manifest<T>,
    ) -> Result<Id<T>, LibraryError> {
        self.insert(manifest.reify())
    }

    pub fn remove(&mut self, id: Id<T>) -> Result<T, LibraryError> {
        self.prototypes
            .remove(&id)
            .ok_or(LibraryError::NotFound(id.to_string()))
    }

    pub fn remove_by_name(&mut self, name: &str) -> Result<T, LibraryError> {
        self.remove(Id::from_name(name))
    }

    pub fn clear(&mut self) {
        self.prototypes.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.prototypes.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum LibraryError {
    /// The name of the item is already in use.
    #[error("Item: {} is already in use.", _0)]
    DuplicateName(String),
    /// The item with the given ID was not found.
    #[error("Item ID: {} was not found.", _0)]
    NotFound(String),
    /// The item with the given name was not found.
    #[error("Item Name: {} was not found.", _0)]
    NameNotFound(String),
}
