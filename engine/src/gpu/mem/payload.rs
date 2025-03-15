use bytemuck::{NoUninit, Pod};
use math::vector::{Vec2, Vec3, Vec4};
use std::num::NonZeroU32;

pub trait ShaderPayload {
    type Output<'a>: NoUninit where Self: 'a;

    fn payload(&self) -> Self::Output<'_>;

    fn count(&self) -> Option<NonZeroU32> { None }
}

pub trait AutoShaderPayload: Copy + NoUninit {}

impl<T: AutoShaderPayload> ShaderPayload for T {
    type Output<'a> = T where Self: 'a;

    fn payload(&self) -> Self::Output<'_> {
        *self
    }
}

impl<T: Pod> AutoShaderPayload for Vec2<T> {}

impl<T: Pod> AutoShaderPayload for Vec3<T> {}

impl<T: Pod> AutoShaderPayload for Vec4<T> {}