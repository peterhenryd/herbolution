/// Allows for the conversion of a type into a [bytemuck::NoUninit] implementation.
///
/// Used by different types of buffers to convert structures into bytes that can be sent to the GPU.
pub trait ToNoUninit {
    type Output: bytemuck::NoUninit;

    fn to_no_uninit(&self) -> Self::Output;
}

impl<T: Copy + bytemuck::NoUninit> ToNoUninit for T {
    type Output = T;

    fn to_no_uninit(&self) -> Self::Output {
        *self
    }
}