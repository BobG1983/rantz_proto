use crate::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use std::fmt::Debug;

/// Trait for Manifests, used to load assets from manifests
pub trait Manifest: Debug + Clone + Asset + for<'de> Deserialize<'de> {
    /// The format of the manifest (json, ron, etc)
    const FORMAT: ManifestFormat;
    /// The prototype this manifest generates
    type Output;
    /// Converts the Manifest into Output
    fn reify(&self) -> Self::Output;
}

pub trait AccessManifestFormat {
    fn manifest_format() -> ManifestFormat;
}

impl<T> AccessManifestFormat for T
where
    T: Manifest,
{
    fn manifest_format() -> ManifestFormat {
        T::FORMAT
    }
}
