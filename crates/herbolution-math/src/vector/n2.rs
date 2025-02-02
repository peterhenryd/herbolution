pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Vector2<U> {
        Vector2::new(f(self.x), f(self.y))
    }
}

impl<T: Copy> Copy for Vector2<T> {}

impl<T: Clone> Clone for Vector2<T> {
    fn clone(&self) -> Self {
        Vector2::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Eq> Eq for Vector2<T> {}

impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Default> Default for Vector2<T> {
    fn default() -> Self {
        Vector2::new(Default::default(), Default::default())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v2({}, {})", self.x, self.y)
    }
}