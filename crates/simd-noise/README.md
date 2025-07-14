# simd-noise

This is a fork of [verpeteren/rust-simd-noise](https://github.com/verpeteren/rust-simd-noise) (`crate::`)
and [arduano/simdeez](https://github.com/arduano/simdeez) (`crate::simd::`). The following changes have been made:

- Support for Neon has been exposed, and the crate doesn't fail to compile on ARM targets;
- noise dimensions are now compile-time constants removing the need for heap allocation (dynamic sizes will be added back later as an option, since heap
  allocation is often desirable);
- overall API changes to better suit the needs of the Herbolution project.
