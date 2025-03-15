use crate::angle::Angle;
use crate::vector::Vec3;
use num::traits::real::Real;
use num::traits::ConstZero;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Euler<A> {
    pub yaw: A,
    pub pitch: A,
    pub roll: A,
}

impl<A> Euler<A> {
    pub const fn new(yaw: A, pitch: A, roll: A) -> Self {
        Self { yaw, pitch, roll }
    }

    pub fn into_view_center(self) -> Vec3<A::Comp>
    where
        A: Angle,
        A::Comp: Real,
    {
        let (sin_pitch, cos_pitch) = self.pitch.into_rad().0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.into_rad().0.sin_cos();
        Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }

    pub fn into_view_directions(self) -> (Vec3<A::Comp>, Vec3<A::Comp>)
    where
        A: Angle,
        A::Comp: Real + ConstZero,
    {
        let (sin_yaw, cos_yaw) = self.yaw.into_rad().0.sin_cos();

        let straight = Vec3::new(cos_yaw, ConstZero::ZERO, sin_yaw);
        let side = Vec3::new(-sin_yaw, ConstZero::ZERO, cos_yaw);

        (straight.normalize(), side.normalize())
    }
}

impl<A: Angle + ConstZero> Euler<A>
{
    pub const IDENTITY: Self = Self::new(A::ZERO, A::ZERO, A::ZERO);
}

impl<A: Angle + ConstZero> Default for Euler<A> {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl<A: Add<Output = A>> Add for Euler<A> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.yaw + rhs.yaw,
            self.pitch + rhs.pitch,
            self.roll + rhs.roll,
        )
    }
}

impl<A: Add<Output = A> + Copy> Add<A> for Euler<A> {
    type Output = Self;

    fn add(self, rhs: A) -> Self::Output {
        Self::new(self.yaw + rhs, self.pitch + rhs, self.roll + rhs)
    }
}

impl<A: AddAssign> AddAssign for Euler<A> {
    fn add_assign(&mut self, rhs: Self) {
        self.yaw += rhs.yaw;
        self.pitch += rhs.pitch;
        self.roll += rhs.roll;
    }
}

impl<A: AddAssign + Copy> AddAssign<A> for Euler<A> {
    fn add_assign(&mut self, rhs: A) {
        self.yaw += rhs;
        self.pitch += rhs;
        self.roll += rhs;
    }
}

impl<A: Sub<Output = A>> Sub for Euler<A> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.yaw - rhs.yaw,
            self.pitch - rhs.pitch,
            self.roll - rhs.roll,
        )
    }
}

impl<A: Sub<Output = A> + Copy> Sub<A> for Euler<A> {
    type Output = Self;

    fn sub(self, rhs: A) -> Self::Output {
        Self::new(self.yaw - rhs, self.pitch - rhs, self.roll - rhs)
    }
}

impl<A: SubAssign> SubAssign for Euler<A> {
    fn sub_assign(&mut self, rhs: Self) {
        self.yaw -= rhs.yaw;
        self.pitch -= rhs.pitch;
        self.roll -= rhs.roll;
    }
}

impl<A: SubAssign + Copy> SubAssign<A> for Euler<A> {
    fn sub_assign(&mut self, rhs: A) {
        self.yaw -= rhs;
        self.pitch -= rhs;
        self.roll -= rhs;
    }
}

impl<A: Mul<Output = A>> Mul for Euler<A> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.yaw * rhs.yaw,
            self.pitch * rhs.pitch,
            self.roll * rhs.roll,
        )
    }
}

impl<A: Mul<Output = A> + Copy> Mul<A> for Euler<A> {
    type Output = Self;

    fn mul(self, rhs: A) -> Self::Output {
        Self::new(self.yaw * rhs, self.pitch * rhs, self.roll * rhs)
    }
}

impl<A: MulAssign> MulAssign for Euler<A> {
    fn mul_assign(&mut self, rhs: Self) {
        self.yaw *= rhs.yaw;
        self.pitch *= rhs.pitch;
        self.roll *= rhs.roll;
    }
}

impl<A: MulAssign + Copy> MulAssign<A> for Euler<A> {
    fn mul_assign(&mut self, rhs: A) {
        self.yaw *= rhs;
        self.pitch *= rhs;
        self.roll *= rhs;
    }
}

impl<A: Div<Output = A>> Div for Euler<A> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.yaw / rhs.yaw,
            self.pitch / rhs.pitch,
            self.roll / rhs.roll,
        )
    }
}

impl<A: Div<Output = A> + Copy> Div<A> for Euler<A> {
    type Output = Self;

    fn div(self, rhs: A) -> Self::Output {
        Self::new(self.yaw / rhs, self.pitch / rhs, self.roll / rhs)
    }
}

impl<A: DivAssign> DivAssign for Euler<A> {
    fn div_assign(&mut self, rhs: Self) {
        self.yaw /= rhs.yaw;
        self.pitch /= rhs.pitch;
        self.roll /= rhs.roll;
    }
}

impl<A: DivAssign + Copy> DivAssign<A> for Euler<A> {
    fn div_assign(&mut self, rhs: A) {
        self.yaw /= rhs;
        self.pitch /= rhs;
        self.roll /= rhs;
    }
}