use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use std::array::IntoIter;
use std::f32::consts::{FRAC_PI_2, PI};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Index, IndexMut, Not, Sub, SubAssign};

use crate::rotation::{Euler, Quat};
use crate::vector::{vec3i, vec3u5, Vec3};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum CubeFace {
    East,

    West,

    Up,

    Down,

    North,

    South,
}

impl CubeFace {
    pub const VALUES: [Self; 6] = [CubeFace::East, CubeFace::West, CubeFace::Up, CubeFace::Down, CubeFace::North, CubeFace::South];

    #[inline]
    pub fn values() -> IntoIter<Self, 6> {
        Self::VALUES.into_iter()
    }

    #[inline]
    pub fn from_normal(position: vec3i) -> Option<Self> {
        match position {
            Vec3 { x: 1, y: 0, z: 0 } => Some(CubeFace::East),
            Vec3 { x: -1, y: 0, z: 0 } => Some(CubeFace::West),
            Vec3 { x: 0, y: 1, z: 0 } => Some(CubeFace::Up),
            Vec3 { x: 0, y: -1, z: 0 } => Some(CubeFace::Down),
            Vec3 { x: 0, y: 0, z: 1 } => Some(CubeFace::North),
            Vec3 { x: 0, y: 0, z: -1 } => Some(CubeFace::South),
            _ => None,
        }
    }

    #[inline]
    pub fn normal(self) -> vec3i {
        match self {
            Self::East => vec3i::new(1, 0, 0),
            Self::West => vec3i::new(-1, 0, 0),
            Self::Up => vec3i::new(0, 1, 0),
            Self::Down => vec3i::new(0, -1, 0),
            Self::North => vec3i::new(0, 0, 1),
            Self::South => vec3i::new(0, 0, -1),
        }
    }

    pub fn rotation(self) -> Quat {
        match self {
            Self::East => Euler::new(0.0, FRAC_PI_2, 0.0).into(),
            Self::West => Euler::new(0.0, -FRAC_PI_2, 0.0).into(),
            Self::Up => Euler::new(-FRAC_PI_2, 0.0, 0.0).into(),
            Self::Down => Euler::new(FRAC_PI_2, 0.0, 0.0).into(),
            Self::North => Euler::new(0.0, 0.0, 0.0).into(),
            Self::South => Euler::new(0.0, PI, 0.0).into(),
        }
    }

    #[inline]
    pub fn inverse(self) -> Self {
        match self {
            Self::East => CubeFace::West,
            Self::West => CubeFace::East,
            Self::Up => CubeFace::Down,
            Self::Down => CubeFace::Up,
            Self::North => CubeFace::South,
            Self::South => CubeFace::North,
        }
    }

    #[inline]
    pub fn orthonormal_basis(self) -> (vec3i, vec3i, vec3i) {
        match self {
            Self::East => (-Vec3::Z, Vec3::Y, Vec3::X),
            Self::West => (Vec3::Z, Vec3::Y, -Vec3::X),
            Self::Up => (Vec3::X, -Vec3::Z, Vec3::Y),
            Self::Down => (Vec3::X, Vec3::Z, -Vec3::Y),
            Self::North => (Vec3::X, Vec3::Y, Vec3::Z),
            Self::South => (-Vec3::X, Vec3::Y, -Vec3::Z),
        }
    }
}

impl Display for CubeFace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CubeFace::East => "East",
                CubeFace::West => "West",
                CubeFace::Up => "Up",
                CubeFace::Down => "Down",
                CubeFace::North => "North",
                CubeFace::South => "South",
            }
        )
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CubeFaces(u8);

impl CubeFaces {
    pub const EAST: Self = Self(1 << 0);
    pub const WEST: Self = Self(1 << 1);
    pub const UP: Self = Self(1 << 2);
    pub const DOWN: Self = Self(1 << 3);
    pub const NORTH: Self = Self(1 << 4);
    pub const SOUTH: Self = Self(1 << 5);

    const MASK: u8 = 0b0011_1111;

    pub fn all() -> Self {
        Self::EAST | Self::WEST | Self::UP | Self::DOWN | Self::NORTH | Self::SOUTH
    }

    pub fn none() -> Self {
        Self(0)
    }

    pub fn is_full(self) -> bool {
        self.0 & Self::MASK == Self::MASK
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn iter(self) -> FaceIter {
        FaceIter { faces: self, index: 0 }
    }

    pub fn to_single(self) -> Option<CubeFace> {
        match self {
            Self::EAST => Some(CubeFace::East),
            Self::WEST => Some(CubeFace::West),
            Self::UP => Some(CubeFace::Up),
            Self::DOWN => Some(CubeFace::Down),
            Self::NORTH => Some(CubeFace::North),
            Self::SOUTH => Some(CubeFace::South),
            _ => None,
        }
    }

    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn set(&mut self, face: CubeFace, active: bool) {
        if active {
            self.0 |= face as u8;
        } else {
            self.0 &= !(face as u8);
        }
    }

    pub fn contains(self, face: CubeFace) -> bool {
        self.0 & face as u8 != 0
    }
}

impl From<u8> for CubeFaces {
    fn from(value: u8) -> Self {
        Self(value & Self::MASK)
    }
}

impl From<CubeFace> for CubeFaces {
    fn from(value: CubeFace) -> Self {
        match value {
            CubeFace::East => CubeFaces::EAST,
            CubeFace::West => CubeFaces::WEST,
            CubeFace::Up => CubeFaces::UP,
            CubeFace::Down => CubeFaces::DOWN,
            CubeFace::North => CubeFaces::NORTH,
            CubeFace::South => CubeFaces::SOUTH,
        }
    }
}

impl Display for CubeFaces {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut iter = self.iter();
        if let Some(face) = iter.next() {
            write!(f, "{face}")?;
        }

        for face in iter {
            write!(f, ", {face}")?;
        }

        write!(f, "]")
    }
}

impl IntoIterator for CubeFaces {
    type Item = CubeFace;
    type IntoIter = FaceIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl BitAnd for CubeFaces {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for CubeFaces {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for CubeFaces {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for CubeFaces {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Not for CubeFaces {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & Self::MASK)
    }
}

impl Add for CubeFaces {
    type Output = Self;

    fn add(self, rhs: CubeFaces) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl AddAssign for CubeFaces {
    fn add_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Add<CubeFace> for CubeFaces {
    type Output = Self;

    fn add(self, rhs: CubeFace) -> Self::Output {
        Self(self.0 | rhs as u8)
    }
}

impl AddAssign<CubeFace> for CubeFaces {
    fn add_assign(&mut self, rhs: CubeFace) {
        self.0 |= rhs as u8;
    }
}

impl Sub for CubeFaces {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self & !rhs
    }
}

impl SubAssign for CubeFaces {
    fn sub_assign(&mut self, rhs: Self) {
        *self &= !rhs;
    }
}

impl Sub<CubeFace> for CubeFaces {
    type Output = Self;

    fn sub(self, rhs: CubeFace) -> Self::Output {
        Self(self.0 & !(rhs as u8))
    }
}

impl SubAssign<CubeFace> for CubeFaces {
    fn sub_assign(&mut self, rhs: CubeFace) {
        self.0 &= !(rhs as u8);
    }
}

#[derive(Debug, Clone)]
pub struct FaceIter {
    faces: CubeFaces,
    index: u8,
}

impl Iterator for FaceIter {
    type Item = CubeFace;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 6 {
            self.index += 1;

            if self.faces.0 & (1 << self.index - 1) != 0 {
                return Some(CubeFace::VALUES[self.index as usize - 1]);
            }
        }

        None
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PerFace<T> {
    pub east: T,
    pub west: T,
    pub up: T,
    pub down: T,
    pub north: T,
    pub south: T,
}

impl<T> Index<u8> for PerFace<T> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.east,
            1 => &self.west,
            2 => &self.up,
            3 => &self.down,
            4 => &self.north,
            5 => &self.south,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<u8> for PerFace<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.east,
            1 => &mut self.west,
            2 => &mut self.up,
            3 => &mut self.down,
            4 => &mut self.north,
            5 => &mut self.south,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> Index<CubeFace> for PerFace<T> {
    type Output = T;

    fn index(&self, index: CubeFace) -> &Self::Output {
        match index {
            CubeFace::East => &self.east,
            CubeFace::West => &self.west,
            CubeFace::Up => &self.up,
            CubeFace::Down => &self.down,
            CubeFace::North => &self.north,
            CubeFace::South => &self.south,
        }
    }
}

impl<T> IndexMut<CubeFace> for PerFace<T> {
    fn index_mut(&mut self, index: CubeFace) -> &mut Self::Output {
        match index {
            CubeFace::East => &mut self.east,
            CubeFace::West => &mut self.west,
            CubeFace::Up => &mut self.up,
            CubeFace::Down => &mut self.down,
            CubeFace::North => &mut self.north,
            CubeFace::South => &mut self.south,
        }
    }
}

impl<T> PerFace<T> {
    pub const fn new(east: T, west: T, up: T, down: T, north: T, south: T) -> Self {
        Self {
            east,
            west,
            up,
            down,
            north,
            south,
        }
    }

    pub const fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            east: value,
            west: value,
            up: value,
            down: value,
            north: value,
            south: value,
        }
    }

    pub fn mapped<F>(mut f: F) -> Self
    where
        F: FnMut(CubeFace) -> T,
    {
        Self {
            east: f(CubeFace::East),
            west: f(CubeFace::West),
            up: f(CubeFace::Up),
            down: f(CubeFace::Down),
            north: f(CubeFace::North),
            south: f(CubeFace::South),
        }
    }

    pub fn as_array(&self) -> &[T; 6]
    where
        T: Pod,
    {
        bytemuck::cast_ref(self)
    }

    pub fn iter(&self) -> PerFaceIter<'_, T> {
        PerFaceIter { value: self, index: 0 }
    }
}

impl<T: Default> Default for PerFace<T> {
    fn default() -> Self {
        Self {
            east: T::default(),
            west: T::default(),
            up: T::default(),
            down: T::default(),
            north: T::default(),
            south: T::default(),
        }
    }
}

unsafe impl<T: Zeroable> Zeroable for PerFace<T> {}

unsafe impl<T: Pod> Pod for PerFace<T> {}

#[derive(Debug, Clone)]
pub struct PerFaceIter<'a, T> {
    value: &'a PerFace<T>,
    index: u8,
}

impl<'a, T> Iterator for PerFaceIter<'a, T> {
    type Item = (CubeFace, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 6 {
            self.index += 1;
            Some((CubeFace::VALUES[self.index as usize], &self.value[self.index]))
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PerFaceU5 {
    ewu: vec3u5,
    dns: vec3u5,
}

impl PerFaceU5 {
    pub const ZERO: Self = Self {
        ewu: vec3u5::ZERO,
        dns: vec3u5::ZERO,
    };

    pub const fn new(east: u8, west: u8, up: u8, down: u8, north: u8, south: u8) -> Self {
        Self {
            ewu: vec3u5::new(east, west, up),
            dns: vec3u5::new(down, north, south),
        }
    }

    pub fn east(&self) -> u8 {
        self.ewu.x()
    }

    pub fn set_east(&mut self, value: u8) {
        self.ewu.set_x(value);
    }

    pub fn west(&self) -> u8 {
        self.ewu.y()
    }

    pub fn set_west(&mut self, value: u8) {
        self.ewu.set_y(value);
    }

    pub fn up(&self) -> u8 {
        self.ewu.z()
    }

    pub fn set_up(&mut self, value: u8) {
        self.ewu.set_z(value);
    }

    pub fn down(&self) -> u8 {
        self.dns.x()
    }

    pub fn set_down(&mut self, value: u8) {
        self.dns.set_x(value);
    }

    pub fn north(&self) -> u8 {
        self.dns.y()
    }

    pub fn set_north(&mut self, value: u8) {
        self.dns.set_y(value);
    }

    pub fn south(&self) -> u8 {
        self.dns.z()
    }

    pub fn set_south(&mut self, value: u8) {
        self.dns.set_z(value);
    }
}
