macro_rules! cellular {
    ("2d", $fn_name: ident, $($f_type:ident)::+, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(
            x: $($f_type)::+,
            y: $($f_type)::+,
            distance_function: crate::intrinsics::scalar::cell_distance_function::CellDistanceFunction,
            return_type: crate::intrinsics::scalar::cell_return_type::CellReturnType,
            jitter: $($f_type)::+,
            seed: $seed_type,
        ) -> $($f_type)::+ {
            crate::functions::$mod::cellular_2d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                distance_function,
                return_type,
                $transmute_from(jitter),
                seed,
            )
            .$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            distance_function: crate::intrinsics::scalar::cell_distance_function::CellDistanceFunction,
            return_type: crate::intrinsics::scalar::cell_return_type::CellReturnType,
            jitter: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::cellular_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                distance_function,
                return_type,
                $transmute_from(jitter),
                seed,
            )
            .$transmute_to()
        }
    };
}

macro_rules! simplex {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, seed: $seed_type) -> $f_type {
            $mod::simplex_1d::<S>($transmute_from(x), seed).$transmute_to()
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, y: $f_type, seed: $seed_type) -> $f_type {
            $mod::simplex_2d::<S>($transmute_from(x), $transmute_from(y), seed).$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, y: $f_type, z: $f_type, seed: $seed_type) -> $f_type {
            $mod::simplex_3d::<S>($transmute_from(x), $transmute_from(y), $transmute_from(z), seed).$transmute_to()
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, y: $f_type, z: $f_type, w: $f_type, seed: $seed_type) -> $f_type {
            $mod::simplex_4d::<S>($transmute_from(x), $transmute_from(y), $transmute_from(z), $transmute_from(w), seed).$transmute_to()
        }
    };
}

macro_rules! fbm {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, lacunarity: $f_type, gain: $f_type, octaves: u8, seed: $seed_type) -> $f_type {
            $mod::fbm_1d::<S>($transmute_from(x), $transmute_from(lacunarity), $transmute_from(gain), octaves, seed).$transmute_to()
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, y: $f_type, lacunarity: $f_type, gain: $f_type, octaves: u8, seed: $seed_type) -> $f_type {
            $mod::fbm_2d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::fbm_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            w: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::fbm_4d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(w),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
}
macro_rules! ridge {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, lacunarity: $f_type, gain: $f_type, octaves: u8, seed: $seed_type) -> $f_type {
            $mod::ridge_1d::<S>($transmute_from(x), $transmute_from(lacunarity), $transmute_from(gain), octaves, seed).$transmute_to()
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, y: $f_type, lacunarity: $f_type, gain: $f_type, octaves: u8, seed: $seed_type) -> $f_type {
            $mod::ridge_2d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::ridge_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            w: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::ridge_4d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(w),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
}

macro_rules! turbulence {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, lacunarity: $f_type, gain: $f_type, octaves: u8, seed: $seed_type) -> $f_type {
            $mod::turbulence_1d::<S>($transmute_from(x), $transmute_from(lacunarity), $transmute_from(gain), octaves, seed).$transmute_to()
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(x: $f_type, y: $f_type, lacunarity: $f_type, gain: $f_type, octaves: u8, seed: $seed_type) -> $f_type {
            $mod::turbulence_2d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::turbulence_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(target_feature = "sse2", target_feature = "sse4.1", target_feature = "avx2"))]
        pub unsafe fn $fn_name<S: crate::simd::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            w: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::turbulence_4d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(w),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
}

macro_rules! get_noise {
    ($call: ident, $fn_name: ident, $f_type: ty, $($mod: ident)::+) => {
        pub unsafe fn $fn_name<const D: $crate::noise::NoiseDim, S: $crate::simd::Simd>(noise_type: &$crate::noise::NoiseType<D>) -> ([$f_type; D.size()],
        $f_type, $f_type)
        {
            $($mod)::+::$call::<D, S>(noise_type)
        }
    };
}

macro_rules! get_noise_scaled {
    ($call: ident, $fn_name: ident, $f_type: ty) => {
        pub unsafe fn $fn_name<const D: $crate::noise::NoiseDim, S: $crate::simd::Simd>(noise_type: &$crate::noise::NoiseType<D>) -> [$f_type; D.size()] {
            let (mut functions, min, max) = $call::<D, S>(noise_type);
            let dim = $crate::noise::DimNoise::dim(noise_type);
            $crate::functions::scale_noise::<S>(dim.min, dim.max, min, max, functions.as_mut_slice());
            functions
        }
    };
}

#[cfg(target_arch = "x86")]
#[cfg(target_feature = "avx2")]
pub mod avx2;
#[cfg(target_arch = "aarch64")]
#[cfg(target_feature = "neon")]
pub mod neon;
pub mod scalar;
#[cfg(target_arch = "x86")]
#[cfg(target_feature = "sse2")]
pub mod sse2;
#[cfg(target_arch = "x86")]
#[cfg(target_feature = "sse4.1")]
pub mod sse41;
