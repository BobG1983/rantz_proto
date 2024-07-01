# rantz_proto

An opinionated rust library for transforming on-disk assets into Entities in [Bevy](https://www.bevyengine.org).

* Manifest (stored on disk as an asset)
  * Can be a single asset, or a list
* Prototype (stored in memory, and serialized/deserialized as save data)
  * Implement Display and EntityCommands then derive Prototype
* PrototypeLibrary (resource from which you can get a prototype by ID or name)
* Command Extension for spawning prototypes from CommandQueue, World, and Commands (including async)
* Support for custom asset loaders
  * Set ManifestFormat::Custom and write your own loader

TODO:

* Support for iyes_progress
* Move main to examples
* Document
