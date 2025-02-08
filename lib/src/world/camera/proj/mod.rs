use math::matrix::mat4f;

pub mod orthographic;
pub mod perspective;

pub trait Proj {
    fn as_mat4f(&self) -> mat4f;
}