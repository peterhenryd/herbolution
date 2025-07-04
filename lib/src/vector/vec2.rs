use crate::vector::Vec3;
use crate::vector::macros::vector;

vector! {
    struct Vec2<T> {
        x(X = 1, 0): T,
        y(Y = 0, 1): T,
    }
    linearize(x, y)
}

impl<T> Vec2<T> {
    pub fn extend(self, z: T) -> Vec3<T> {
        Vec3 { x: self.x, y: self.y, z }
    }
}
