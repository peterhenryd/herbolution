#[derive(derive::Vector)]
pub struct Color3<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Color4<T> {
    pub fn extend(self, alpha: T) -> Color4<T> {
        Color4::new(self.r, self.g, self.b, alpha)
    }

    pub fn opaque(self) -> Color4<T>
    where
        T: ColorComp,
    {
        self.extend(T::MAX)
    }
}

impl<T: ColorComp> Color3<T> {
    pub const WHITE: Self = Self::new(T::MAX, T::MAX, T::MAX);
    pub const GRAY: Self = Self::new(T::HALF, T::HALF, T::HALF);
    pub const BLACK: Self = Self::new(T::MIN, T::MIN, T::MIN);

    pub const RED: Self = Self::new(T::MAX, T::MIN, T::MIN);
    pub const GREEN: Self = Self::new(T::MIN, T::MAX, T::MIN);
    pub const BLUE: Self = Self::new(T::MIN, T::MIN, T::MAX);

    pub const YELLOW: Self = Self::new(T::MAX, T::MAX, T::MIN);
    pub const CYAN: Self = Self::new(T::MIN, T::MAX, T::MAX);
    pub const MAGENTA: Self = Self::new(T::MAX, T::MIN, T::MAX);

    pub const ORANGE: Self = Self::new(T::MAX, T::HALF, T::MIN);
    pub const PURPLE: Self = Self::new(T::HALF, T::MIN, T::MAX);
    pub const LIME: Self = Self::new(T::MIN, T::MAX, T::HALF);

    pub const PINK: Self = Self::new(T::MAX, T::HALF, T::HALF);
    pub const TEAL: Self = Self::new(T::HALF, T::MAX, T::HALF);
    pub const LAVENDER: Self = Self::new(T::HALF, T::HALF, T::MAX);

    pub const BROWN: Self = Self::new(T::HALF, T::HALF, T::MIN);
    pub const OLIVE: Self = Self::new(T::HALF, T::MAX, T::HALF);
    pub const MAROON: Self = Self::new(T::HALF, T::MIN, T::HALF);
}

#[derive(derive::Vector)]
pub struct Color4<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: ColorComp> Color4<T> {
    pub const TRANSPARENT: Self = Self::new(T::MIN, T::MIN, T::MIN, T::MIN);

    pub const WHITE: Self = Self::new(T::MAX, T::MAX, T::MAX, T::MAX);
    pub const GRAY: Self = Self::new(T::HALF, T::HALF, T::HALF, T::MAX);
    pub const BLACK: Self = Self::new(T::MIN, T::MIN, T::MIN, T::MAX);

    pub const RED: Self = Self::new(T::MAX, T::MIN, T::MIN, T::MAX);
    pub const GREEN: Self = Self::new(T::MIN, T::MAX, T::MIN, T::MAX);
    pub const BLUE: Self = Self::new(T::MIN, T::MIN, T::MAX, T::MAX);

    pub const YELLOW: Self = Self::new(T::MAX, T::MAX, T::MIN, T::MAX);
    pub const CYAN: Self = Self::new(T::MIN, T::MAX, T::MAX, T::MAX);
    pub const MAGENTA: Self = Self::new(T::MAX, T::MIN, T::MAX, T::MAX);

    pub const ORANGE: Self = Self::new(T::MAX, T::HALF, T::MIN, T::MAX);
    pub const PURPLE: Self = Self::new(T::HALF, T::MIN, T::MAX, T::MAX);
    pub const LIME: Self = Self::new(T::MIN, T::MAX, T::HALF, T::MAX);

    pub const PINK: Self = Self::new(T::MAX, T::HALF, T::HALF, T::MAX);
    pub const TEAL: Self = Self::new(T::HALF, T::MAX, T::HALF, T::MAX);
    pub const LAVENDER: Self = Self::new(T::HALF, T::HALF, T::MAX, T::MAX);

    pub const BROWN: Self = Self::new(T::HALF, T::HALF, T::MIN, T::MAX);
    pub const OLIVE: Self = Self::new(T::HALF, T::MAX, T::HALF, T::MAX);
    pub const MAROON: Self = Self::new(T::HALF, T::MIN, T::HALF, T::MAX);
}

pub trait ColorComp {
    const MIN: Self;
    const HALF: Self;
    const MAX: Self;
}

impl ColorComp for u8 {
    const MIN: Self = u8::MIN;
    const HALF: Self = u8::MAX / 2;
    const MAX: Self = u8::MAX;
}

impl ColorComp for f32 {
    const MIN: Self = 0.0;
    const HALF: Self = 0.5;
    const MAX: Self = 1.0;
}

impl ColorComp for f64 {
    const MIN: Self = 0.0;
    const HALF: Self = 0.5;
    const MAX: Self = 1.0;
}