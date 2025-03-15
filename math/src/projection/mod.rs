use crate::matrix::mat4f;

pub mod orthographic;
pub mod perspective;

pub trait Projection {
    fn to_matrix(&self) -> mat4f;
}