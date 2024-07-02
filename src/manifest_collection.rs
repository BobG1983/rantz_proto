use crate::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, Asset, TypePath, Debug)]
pub struct ManifestCollection<M>
where
    M: Asset + Clone,
{
    pub prototypes: Vec<M>,
}

impl<M> AccessManifestFormat for ManifestCollection<M>
where
    M: Manifest,
{
    fn manifest_format() -> ManifestFormat {
        M::FORMAT
    }
}

impl<M> Default for ManifestCollection<M>
where
    M: Manifest,
{
    fn default() -> Self {
        Self {
            prototypes: Default::default(),
        }
    }
}

impl<M> IntoIterator for ManifestCollection<M>
where
    M: Manifest,
{
    type Item = M;

    type IntoIter = ManifestCollectionIntoIterator<M>;

    fn into_iter(self) -> Self::IntoIter {
        ManifestCollectionIntoIterator {
            collection: self,
            index: 0,
        }
    }
}

impl<'a, M> IntoIterator for &'a ManifestCollection<M>
where
    M: Manifest,
{
    type Item = M;

    type IntoIter = ManifestCollectionRefIntoIterator<'a, M>;

    fn into_iter(self) -> Self::IntoIter {
        ManifestCollectionRefIntoIterator {
            collection: self,
            index: 0,
        }
    }
}

pub struct ManifestCollectionIntoIterator<M>
where
    M: Manifest,
{
    collection: ManifestCollection<M>,
    index: usize,
}

impl<M> Iterator for ManifestCollectionIntoIterator<M>
where
    M: Manifest,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collection.prototypes.len() {
            self.index += 1;
            Some(self.collection.prototypes[self.index - 1].clone())
        } else {
            None
        }
    }
}

pub struct ManifestCollectionRefIntoIterator<'a, M>
where
    M: Manifest,
{
    collection: &'a ManifestCollection<M>,
    index: usize,
}

impl<'a, M> Iterator for ManifestCollectionRefIntoIterator<'a, M>
where
    M: Manifest,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collection.prototypes.len() {
            self.index += 1;
            Some(self.collection.prototypes[self.index - 1].clone())
        } else {
            None
        }
    }
}
