#[derive(Debug)]
pub struct Plane<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

impl<T> Plane<T> {
    pub const fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }
}

impl<T: Copy + num::Float> Plane<T> {
    pub fn normalize(self) -> Self {
        let l = (self.a.powi(2) + self.b.powi(2) + self.c.powi(2)).sqrt();
        Self {
            a: self.a / l,
            b: self.b / l,
            c: self.c / l,
            d: self.d / l,
        }
    }
}

impl<T: Copy> Copy for Plane<T> {}

impl<T: Clone> Clone for Plane<T> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
            d: self.d.clone(),
        }
    }
}

impl<T: Default> Default for Plane<T> {
    fn default() -> Self {
        Self {
            a: T::default(),
            b: T::default(),
            c: T::default(),
            d: T::default(),
        }
    }
}