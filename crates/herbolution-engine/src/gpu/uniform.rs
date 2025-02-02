use bytemuck::NoUninit;

pub trait UniformObject {
    fn get_raw(&self) -> impl NoUninit;
}
