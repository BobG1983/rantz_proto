use bevy_asset::Asset;
use serde::Deserialize;

pub trait Manifest<T>: Asset + for<'de> Deserialize<'de> {
    fn reify(&self) -> T;
}
