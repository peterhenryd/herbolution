mod degree;
mod radian;

pub use degree::Deg;
pub use radian::Rad;

pub trait Angle {
    type Comp;

    fn into_deg(self) -> Deg<Self::Comp>;

    fn into_rad(self) -> Rad<Self::Comp>;
}
