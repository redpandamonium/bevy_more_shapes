# bevy_more_shapes
![crates.io](https://img.shields.io/crates/v/bevy_more_shapes.svg)

More shapes for the bevy game engine. This plugin adds more procedural geometry shapes for bevy.
It works exactly like the default bevy shapes. 

To run the example showcasing all the available shapes, run `cargo run --example gallery`.

## Features

* Cones
* Cylinders
* Grid planes
* Arbitrary non-self-intersecting polygons
* Torus

Planned:

* Torus segments

## Versions

This crate tracks bevy's versions, meaning the API of this crate will only break when an API breaking version of bevy is released.

| Version | Bevy version |
|---------|--------------|
| 0.1.x   | 0.6.x        |
| 0.2.x   | 0.7.x        |

## Known Issues

The UV coordinates are correct but not always useful. Repeating textures make this better, but consider changing the UVs for your use-case.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as stated in the LICENSE file, without any additional terms or conditions.
