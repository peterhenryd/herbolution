#![allow(non_camel_case_types)]

mod ext2;

pub use ext2::Ext2;

pub type ext2u8 = Ext2<u8>;
pub type ext2u16 = Ext2<u16>;
pub type ext2u = Ext2<u32>;
pub type ext2u64 = Ext2<u64>;
pub type ext2u128 = Ext2<u128>;
pub type ext2usize = Ext2<usize>;
pub type ext2i8 = Ext2<i8>;
pub type ext2i16 = Ext2<i16>;
pub type ext2i32 = Ext2<i32>;
pub type ext2i64 = Ext2<i64>;
pub type ext2i128 = Ext2<i128>;
pub type ext2isize = Ext2<isize>;
pub type ext2f = Ext2<f32>;
pub type ext2d = Ext2<f64>;