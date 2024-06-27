use crate::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct ManifestCollection<M>
where
    M: Asset + Clone,
{
    pub protos: Vec<M>,
}

impl<M> Default for ManifestCollection<M>
where
    M: Manifest,
{
    fn default() -> Self {
        Self {
            protos: Default::default(),
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
            collection: self.protos,
        }
    }
}

pub struct ManifestCollectionIntoIterator<M>
where
    M: Manifest,
{
    collection: Vec<M>,
}

impl<M> Iterator for ManifestCollectionIntoIterator<M>
where
    M: Manifest,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        self.collection.pop()
    }
}
