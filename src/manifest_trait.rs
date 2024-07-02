use crate::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use std::fmt::Debug;

// pub trait Manifest<P>: Asset + for<'de> Deserialize<'de>
// where
//     P: Prototype,
// {
//     const FORMAT: ManifestFormat;
//     fn reify(&self) -> P;
// }

pub trait Manifest: Debug + Clone + Asset + for<'de> Deserialize<'de> {
    const FORMAT: ManifestFormat;
    type Output;
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
