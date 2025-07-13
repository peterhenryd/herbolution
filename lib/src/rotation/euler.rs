use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num::traits::ConstZero;
use num::Float;
use serde::{Deserialize, Serialize};

use crate::vector::Vec3;

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

    pub fn into_view_center(self) -> Vec3<A>
    where
        A: Float + ConstZero,
    {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }

    pub fn yaw_directions(self) -> (Vec3<A>, Vec3<A>)
    where
        A: Float + ConstZero,
    {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        let straight = Vec3::new(cos_yaw, A::ZERO, sin_yaw);
        let side = Vec3::new(-sin_yaw, A::ZERO, cos_yaw);

        (straight.normalize(), side.normalize())
    }
}

impl<A: ConstZero> Euler<A> {
    pub const IDENTITY: Self = Self::new(A::ZERO, A::ZERO, A::ZERO);
}

impl<A: ConstZero> Default for Euler<A> {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl<A> Add for Euler<A>
where
    A: Add<Output = A>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.yaw + rhs.yaw, self.pitch + rhs.pitch, self.roll + rhs.roll)
    }
}

impl<A> Add<A> for Euler<A>
where
    A: Add<Output = A> + Copy,
{
    type Output = Self;

    fn add(self, rhs: A) -> Self::Output {
        Self::new(self.yaw + rhs, self.pitch + rhs, self.roll + rhs)
    }
}

impl<A> AddAssign for Euler<A>
where
    A: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.yaw += rhs.yaw;
        self.pitch += rhs.pitch;
        self.roll += rhs.roll;
    }
}

impl<A> AddAssign<A> for Euler<A>
where
    A: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: A) {
        self.yaw += rhs;
        self.pitch += rhs;
        self.roll += rhs;
    }
}

impl<A> Sub for Euler<A>
where
    A: Sub<Output = A>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.yaw - rhs.yaw, self.pitch - rhs.pitch, self.roll - rhs.roll)
    }
}

impl<A> Sub<A> for Euler<A>
where
    A: Sub<Output = A> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: A) -> Self::Output {
        Self::new(self.yaw - rhs, self.pitch - rhs, self.roll - rhs)
    }
}

impl<A> SubAssign for Euler<A>
where
    A: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.yaw -= rhs.yaw;
        self.pitch -= rhs.pitch;
        self.roll -= rhs.roll;
    }
}

impl<A> SubAssign<A> for Euler<A>
where
    A: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: A) {
        self.yaw -= rhs;
        self.pitch -= rhs;
        self.roll -= rhs;
    }
}

impl<A> Mul for Euler<A>
where
    A: Mul<Output = A>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.yaw * rhs.yaw, self.pitch * rhs.pitch, self.roll * rhs.roll)
    }
}

impl<A> Mul<A> for Euler<A>
where
    A: Mul<Output = A> + Copy,
{
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

impl<A> Div for Euler<A>
where
    A: Div<Output = A>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.yaw / rhs.yaw, self.pitch / rhs.pitch, self.roll / rhs.roll)
    }
}

impl<A> Div<A> for Euler<A>
where
    A: Div<Output = A> + Copy,
{
    type Output = Self;

    fn div(self, rhs: A) -> Self::Output {
        Self::new(self.yaw / rhs, self.pitch / rhs, self.roll / rhs)
    }
}

impl<A> DivAssign for Euler<A>
where
    A: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.yaw /= rhs.yaw;
        self.pitch /= rhs.pitch;
        self.roll /= rhs.roll;
    }
}

impl<A> DivAssign<A> for Euler<A>
where
    A: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: A) {
        self.yaw /= rhs;
        self.pitch /= rhs;
        self.roll /= rhs;
    }
}
