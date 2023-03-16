# Migration Guide

This guide documents all breaking changes as well as how to migrate from one version to the next.
If a version is not in here, there were no breaking changes.

## 0.3.x -> 0.4.0

* The Torus shape has had its fields renamed to avoid confusion. Horizontal -> Radial, Vertical -> Tube
* The Torus shape has gained 2 parameters, set them to 2pi for the old behavior
* The default Torus has doubled its segments but is the same otherwise
* Polygon now implements Mesh::try_from instead of Mesh::from, because the underlying library now returns an error if the input data was malformed