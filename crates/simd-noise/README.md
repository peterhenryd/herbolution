# rust-simd-noise

This is a fork of [verpeteren/rust-simd-noise](https://github.com/verpeteren/rust-simd-noise) and [arduano/simdeez](https://github.com/arduano/simdeez). I
have made the following changes to both:

- "finished" (exposed as a public API, really) support for ARM Neon,
- noise dimensions are now compile-time constants removing the need for heap allocation (dynamic sizes will be added back later),
- overall API changes to what I personally find to be more ergonomic.
