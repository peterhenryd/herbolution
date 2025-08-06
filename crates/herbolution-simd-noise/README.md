# herbolution-simd-noise

This is a fork of [verpeteren/rust-simd-noise](https://github.com/verpeteren/rust-simd-noise) (`crate::`)
and [arduano/simdeez](https://github.com/arduano/simdeez) (`crate::simd::`). The following changes have been made:
* Removed files and portions of files that are not necessary for compilation (but preserved original license).
* support for Neon has been exposed for compatbility with ARM targets;
* noise dimensions are now compile-time constants;
* noise functions return sized arrays instead of dynamically allocated vectors;
* overall API changes to better suit the needs of the Herbolution project.