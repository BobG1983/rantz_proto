# rantz_proto

An opinionated rust library for transforming on-disk assets into Entities in [Bevy](https://www.bevyengine.org).

## Manifests

Manifests are stored on disk in one of many different formats (see 'Cargo.toml'). They are loaded and transformed into a struct that implements the Manifest trait. These structs are then further transformed into Prototypes and stored in a PrototypeLibrary.

## Prototypes

Prototypes are structs that implement the Prototype trait. They are stored in a PrototypeLibrary resource. The trait has two primary functions, name() and build(). name() is used to generate the unique name which is used to retrieve the prototype from the library. build() is responsible for turning a prototype into an entity. The trait has an optional third function, rebuild(), a default implementation is provided that does nothing.

**Note**
A single struct can implement both Manifest and Prototype and just return Self in the manifest trait.

## Hot Reloading
**Note**: Requires 'hot_reloading' (on by default)

If an underlying asset is changed the manifests are reloaded and reprocessed into prototypes, then each entity that was built with a given prototype is passed to rebuild() to allow for "on-the-fly" changes.

Basically, if you use a prototype rebuild() will let you change any live entities to match the changes.

## Registering

In order for a manifest to be loaded and processed it must be registered. This is done as part of app building by calling 'app.add_prototype()'

## Loading Progress
**Note**: Requires 'progress_tracking' (on by default)

This crate provides support for iyes_progress for tracking loading progress. If you wish to control when loading happens it is possible to use configure_set with 'ProtoSchedule::Loading' to control when loading happens. 

## Spawning Prototypes

Extensions to 'Commands', 'World' and 'CommandQueue' are provided that introduce 'spawn_prototype' and 'spawn_prototype_async' that allow for easy spawning of prototypes.

## Spawning at a Location

I didn't want to be specific about how locations were handled, so this is left up to you. My recommendation is to add a position to your prototypes, and edit that position before calling spawn_prototype.

## Usage

Add the ProtoPlugin to your app. Register your manifests using app.add_manifest() then spawn entities with commands.spawn_prototype().

If you want to control when load happens, use app.configure_sets() on the ProtoSchedule::Loading schedule.
