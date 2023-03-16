# Migration Guide

This document documents the history of all breaking changes.

## 0.3.x -> 0.4.0

* The Torus shape has had its fields renamed to avoid confusion. Horizontal -> Radial, Vertical -> Tube
* The Torus shape has gained 2 parameters, set them to Default::default() for the old behavior
* The default Torus has doubled its segments but is the same otherwise