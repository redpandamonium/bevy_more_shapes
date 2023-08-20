# bevy_more_shapes
![crates.io](https://img.shields.io/crates/v/bevy_more_shapes.svg)

![Gallery Screenshot](https://github.com/redpandamonium/bevy_more_shapes/blob/97220661580d93c0e53ce1a0ae68cd02d4fa2cda/assets/screenshots/screenshot.png)

More shapes for the bevy game engine. This plugin adds more procedural geometry shapes for bevy.
It works exactly like the default bevy shapes. 

To run the example showcasing all the available shapes, run `cargo run --example gallery`.

## Features

* Cones
* Cylinders
* Grid planes
* Arbitrary non-self-intersecting polygons
* Torus (Including segmented torus)
* Tubes that follow an arbitrary 3d curve

## Versions

This crate tracks bevy's versions. It also follows the semver standard.
Below is a chart which versions of this crate are compatible with which bevy version:

| Version | Bevy version |
|---------|--------------|
| 0.1.x   | 0.6.x        |
| 0.2.x   | 0.7.x        |
| 0.3.x   | 0.9.x        |
| 0.4.x   | 0.10.x       |
| 0.5.x   | 0.10.x       |
| 0.6.x   | 0.11.x       |

## Known Issues

The normals on cones and cylinders aren't properly smoothly interpolated. 

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as stated in the LICENSE file, without any additional terms or conditions.
