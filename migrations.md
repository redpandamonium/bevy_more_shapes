# Migration Guide

This guide documents all breaking changes as well as how to migrate from one version to the next.
If a version is not in here, there were no breaking changes.

## 0.3.x -> 0.4.0

* The Torus shape has had its fields renamed to avoid confusion. Horizontal -> Radial, Vertical -> Tube
* The Torus shape has gained 2 parameters, set them to 2pi for the old behavior
* The default Torus has doubled its segments but is the same otherwise
* Polygon now implements Mesh::try_from instead of Mesh::from, because the underlying library now returns an error if the input data was malformed

## 0.4 -> 0.5

* The cylinder shape supports height segments now. Default is 1 which is the old behavior.
* The cylinder UVs have been reworked to make more sense
* The cylinder normals have been fixed to account for the slope on irregular cylinders
* Cone segment parameter was renamed
* Cone UVs were redone to make more sense
* Cone normals have been fixed to account for the slope