/// Allows for the conversion of a type into a [bytemuck::NoUninit] implementation.
///
/// Used by different types of buffers to convert structures into bytes that can be sent to the GPU.
pub trait AsNoUninit {
    type Output: bytemuck::NoUninit;

    fn as_no_uninit(&self) -> Self::Output;
}

impl<T: Copy + bytemuck::NoUninit> AsNoUninit for T {
    type Output = T;

    fn as_no_uninit(&self) -> Self::Output {
        *self
    }
}