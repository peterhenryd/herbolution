#![allow(non_camel_case_types)]

crate::reexport! {
    mod size2;
    mod size3;
}

pub type size2u8 = Size2<u8>;
pub type size2u16 = Size2<u16>;
pub type size2u = Size2<u32>;
pub type size2u64 = Size2<u64>;
pub type size2u128 = Size2<u128>;
pub type size2usize = Size2<usize>;
pub type size2i8 = Size2<i8>;
pub type size2i16 = Size2<i16>;
pub type size2i32 = Size2<i32>;
pub type size2i64 = Size2<i64>;
pub type size2i128 = Size2<i128>;
pub type size2isize = Size2<isize>;
pub type size2f = Size2<f32>;
pub type size2d = Size2<f64>;

pub type size3u8 = Size3<u8>;
pub type size3u16 = Size3<u16>;
pub type size3u = Size3<u32>;
pub type size3u64 = Size3<u64>;
pub type size3u128 = Size3<u128>;
pub type size3usize = Size3<usize>;
pub type size3i8 = Size3<i8>;
pub type size3i16 = Size3<i16>;
pub type size3i32 = Size3<i32>;
pub type size3i64 = Size3<i64>;
pub type size3i128 = Size3<i128>;
pub type size3isize = Size3<isize>;
pub type size3f = Size3<f32>;
pub type size3d = Size3<f64>;
