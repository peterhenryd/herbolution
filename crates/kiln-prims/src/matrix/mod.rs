#![allow(non_camel_case_types)]

pub use mat3::Mat3;
pub use mat4::Mat4;

mod mat3;
mod mat4;

pub type mat3f = Mat3<f32>;
pub type mat3d = Mat3<f64>;

pub type mat4f = Mat4<f32>;
pub type mat4d = Mat4<f64>;
