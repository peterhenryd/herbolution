use bytemuck::NoUninit;

pub trait Payload: NoUninit {
    type Source;

    fn from_source(source: &Self::Source) -> Self;
}

impl Payload for u16 {
    type Source = u16;

    fn from_source(source: &Self::Source) -> Self {
        *source
    }
}

impl Payload for u32 {
    type Source = u32;

    fn from_source(source: &Self::Source) -> Self {
        *source
    }
}
