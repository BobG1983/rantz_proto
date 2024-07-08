use crate::prelude::*;
use bevy::{prelude::*, utils::HashMap};

/// A generic library of prototypes. Accessed as a resource when you want
/// to load prototypes of type P.
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
    /// Create a new PrototypeLibrary
    pub fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    /// Gets a prototype from the library by id, or None if the prototype doesn't exist
    #[must_use]
    pub fn get(&self, id: &Id<P>) -> Option<P> {
        if let Some(p) = self.prototypes.get(id) {
            let p: P = p.to_owned();
            Some(p)
        } else {
            None
        }
    }

    /// Gets a prototype from the library by name, or None if the prototype doesn't exist
    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<P> {
        self.get(&Id::from_name(name))
    }

    /// Gets the id of a prototype in the library by name, or None if the
    /// prototype doesn't exist
    #[must_use]
    pub fn get_id(&self, name: &str) -> Option<Id<P>> {
        let maybe_id = Id::from_name(name);
        if self.get(&maybe_id).is_some() {
            return Some(maybe_id);
        }

        None
    }

    /// Gets mutable access to a prototype from the library by id,
    /// or None if the prototype doesn't exist
    #[must_use]
    pub fn get_mut(&mut self, id: &Id<P>) -> Option<&mut P> {
        self.prototypes.get_mut(id)
    }

    /// Gets mutable access to a prototype from the library by name,
    /// or None if the prototype doesn't exist
    #[must_use]
    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut P> {
        self.get_mut(&Id::from_name(name))
    }

    pub(crate) fn insert(&mut self, item: P) -> Id<P> {
        let id = Id::from_name(&item.name());
        self.prototypes.insert(id, item);
        id
    }

    pub(crate) fn clear(&mut self) {
        self.prototypes.clear();
    }

    /// Returns true if the library is empty
    pub fn is_empty(&self) -> bool {
        self.prototypes.is_empty()
    }

    /// Returns the number of prototypes in the library
    pub fn len(&self) -> usize {
        self.prototypes.len()
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
