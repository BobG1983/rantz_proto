use crate::prelude::*;
use bevy::{prelude::*, utils::HashMap};
use thiserror::Error;

#[derive(Debug, Resource)]
pub struct PrototypeLibrary<P>
where
    P: Prototype,
{
    prototypes: HashMap<Id<P>, P>,
}

impl<P> PrototypeLibrary<P>
where
    P: Prototype,
{
    pub fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    #[must_use]
    pub fn get(&self, id: &Id<P>) -> Option<P> {
        if let Some(p) = self.prototypes.get(id) {
            let p: P = p.to_owned();
            Some(p)
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<P> {
        self.get(&Id::from_name(name))
    }

    #[must_use]
    pub fn get_id(&self, name: &str) -> Option<Id<P>> {
        let maybe_id = Id::from_name(name);
        if self.get(&maybe_id).is_some() {
            return Some(maybe_id);
        }

        None
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &Id<P>) -> Option<&mut P> {
        self.prototypes.get_mut(id)
    }

    #[must_use]
    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut P> {
        self.get_mut(&Id::from_name(name))
    }

    pub(crate) fn insert(&mut self, item: P) -> Id<P> {
        let id = Id::from_name(&item.to_string());
        self.prototypes.insert(id, item);
        id
    }

    pub(crate) fn clear(&mut self) {
        self.prototypes.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.prototypes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.prototypes.len()
    }

    pub fn first(&self) -> Option<P> {
        if let Some(key) = self.prototypes.keys().next() {
            self.get(key)
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<P> {
        if let Some(key) = self.prototypes.keys().last() {
            self.get(key)
        } else {
            None
        }
    }
}

impl<T> Default for PrototypeLibrary<T>
where
    T: Prototype,
{
    fn default() -> Self {
        Self {
            prototypes: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PrototypeLibraryError {
    /// The name of the item is already in use.
    #[error("Item: {} is already in use.", _0)]
    Duplicate(String),
    /// The item was not found.
    #[error("Item: {} was not found.", _0)]
    NotFound(String),
}
