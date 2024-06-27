/// The file format of the raw manifest on disk.
///
/// All of the corresponding features are off by default, and must be enabled with feature flags.
/// Check the `Cargo.toml` file for the list of available features.
pub enum ManifestFormat {
    #[cfg(feature = "ron")]
    /// A Rust-specific configuration format that is easy for both humans and machines to read and write.
    Ron,
    #[cfg(feature = "json")]
    /// A standard configuration format that is easy for both humans and machines to read and write.
    Json,
    #[cfg(feature = "yaml")]
    /// A configuration format that accepts complex data structures, with a focus on human-editable data.
    Yaml,
    #[cfg(feature = "toml")]
    /// A configuration format that emphasizes readability and simplicity, with a focus on human-editable data.
    Toml,
    #[cfg(feature = "xml")]
    /// A markup language that defines a set of rules for encoding documents in a format that is both human-readable and machine-readable.
    Xml,
    #[cfg(feature = "csv")]
    /// A simple text-based tabular format, with rows separated by newlines and columns separated by commas.
    Csv,
    #[cfg(feature = "msgpack")]
    /// A JSON-derived binary format.
    MsgPack,
    /// Your own custom format.
    ///
    /// If this is selected, you will need to create and register your own [`bevy::asset::AssetLoader`] trait for the [`Manifest`] asset type.
    Custom,
}
