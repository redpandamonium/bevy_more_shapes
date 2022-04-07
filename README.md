# bevy_more_shapes
More shapes for the bevy game engine. This plugin adds more procedural geometry shapes for bevy.
It works exactly like the default bevy shapes. 

To run the example showcasing all the available shapes, run `cargo run --example gallery`.

This crate tracks bevy's versions, meaning the API of this crate will only break when an API breaking version of bevy is released. It is currently compatible with versions 0.6 of bevy.

## Features

* Cones

Planned, coming soon: 

* Torus
* Cylinder

## Known issues

The demo's camera sometimes jumps when you start moving the mouse because the first movement delta is sometimes huge. 
This is an issue with the camera plugin used.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.