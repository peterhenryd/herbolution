use std::array::IntoIter;
use std::f32::consts::{FRAC_PI_2, PI};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

use crate::rotation::{Euler, Quat};
use crate::vector::{vec3i, vec3u5, Vec3};
use bytemuck::{Pod, Zeroable};
use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use serde::{Deserialize, Serialize};

// Face

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Face {
    // Positive X
    East,
    // Negative X
    West,
    // Positive Y
    Up,
    // Negative Y
    Down,
    // Positive Z
    North,
    // Negative Z
    South,
}

impl Face {
    pub const VALUES: [Self; 6] = [Face::East, Face::West, Face::Up, Face::Down, Face::North, Face::South];

    #[inline]
    pub fn values() -> IntoIter<Self, 6> {
        Self::VALUES.into_iter()
    }

    #[inline]
    pub fn from_normal(position: vec3i) -> Option<Self> {
        match position {
            Vec3 { x: 1, y: 0, z: 0 } => Some(Face::East),
            Vec3 { x: -1, y: 0, z: 0 } => Some(Face::West),
            Vec3 { x: 0, y: 1, z: 0 } => Some(Face::Up),
            Vec3 { x: 0, y: -1, z: 0 } => Some(Face::Down),
            Vec3 { x: 0, y: 0, z: 1 } => Some(Face::North),
            Vec3 { x: 0, y: 0, z: -1 } => Some(Face::South),
            _ => None,
        }
    }

    #[inline]
    pub fn to_normal(self) -> vec3i {
        match self {
            Face::East => vec3i::new(1, 0, 0),
            Face::West => vec3i::new(-1, 0, 0),
            Face::Up => vec3i::new(0, 1, 0),
            Face::Down => vec3i::new(0, -1, 0),
            Face::North => vec3i::new(0, 0, 1),
            Face::South => vec3i::new(0, 0, -1),
        }
    }

    pub fn to_rotation(self) -> Quat {
        match self {
            Face::East => Euler::new(0.0, FRAC_PI_2, 0.0).into(),
            Face::West => Euler::new(0.0, -FRAC_PI_2, 0.0).into(),
            Face::Up => Euler::new(-FRAC_PI_2, 0.0, 0.0).into(),
            Face::Down => Euler::new(FRAC_PI_2, 0.0, 0.0).into(),
            Face::North => Euler::new(0.0, 0.0, 0.0).into(),
            Face::South => Euler::new(0.0, PI, 0.0).into(),
        }
    }

    #[inline]
    pub fn inverse(self) -> Self {
        match self {
            Face::East => Face::West,
            Face::West => Face::East,
            Face::Up => Face::Down,
            Face::Down => Face::Up,
            Face::North => Face::South,
            Face::South => Face::North,
        }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Face::East => "East",
                Face::West => "West",
                Face::Up => "Up",
                Face::Down => "Down",
                Face::North => "North",
                Face::South => "South",
            }
        )
    }
}

// Faces

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, BitAnd, BitAndAssign, BitOr, BitOrAssign, Not)]
pub struct Faces(u8);

const MASK: u8 = 0b0011_1111;

impl Faces {
    pub const EAST: Self = Self(1 << 0);
    pub const WEST: Self = Self(1 << 1);
    pub const UP: Self = Self(1 << 2);
    pub const DOWN: Self = Self(1 << 3);
    pub const NORTH: Self = Self(1 << 4);
    pub const SOUTH: Self = Self(1 << 5);

    pub fn all() -> Self {
        Self::EAST | Self::WEST | Self::UP | Self::DOWN | Self::NORTH | Self::SOUTH
    }

    pub fn none() -> Self {
        Self(0)
    }

    pub fn is_full(self) -> bool {
        self.0 & MASK == MASK
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn iter(self) -> FaceIter {
        FaceIter { faces: self, index: 0 }
    }

    pub fn to_single(self) -> Option<Face> {
        match self {
            Self::EAST => Some(Face::East),
            Self::WEST => Some(Face::West),
            Self::UP => Some(Face::Up),
            Self::DOWN => Some(Face::Down),
            Self::NORTH => Some(Face::North),
            Self::SOUTH => Some(Face::South),
            _ => None,
        }
    }

    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn set(&mut self, face: Face, active: bool) {
        if active {
            self.0 |= face as u8;
        } else {
            self.0 &= !(face as u8);
        }
    }

    pub fn contains(self, face: Face) -> bool {
        self.0 & face as u8 != 0
    }
}

impl From<u8> for Faces {
    fn from(value: u8) -> Self {
        Self(value & MASK)
    }
}

impl From<Face> for Faces {
    fn from(value: Face) -> Self {
        match value {
            Face::East => Faces::EAST,
            Face::West => Faces::WEST,
            Face::Up => Faces::UP,
            Face::Down => Faces::DOWN,
            Face::North => Faces::NORTH,
            Face::South => Faces::SOUTH,
        }
    }
}

impl Display for Faces {
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

impl IntoIterator for Faces {
    type Item = Face;
    type IntoIter = FaceIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Add for Faces {
    type Output = Self;

    fn add(self, rhs: Faces) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl AddAssign for Faces {
    fn add_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Add<Face> for Faces {
    type Output = Self;

    fn add(self, rhs: Face) -> Self::Output {
        Self(self.0 | rhs as u8)
    }
}

impl AddAssign<Face> for Faces {
    fn add_assign(&mut self, rhs: Face) {
        self.0 |= rhs as u8;
    }
}

impl Sub for Faces {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self & !rhs
    }
}

impl SubAssign for Faces {
    fn sub_assign(&mut self, rhs: Self) {
        *self &= !rhs;
    }
}

impl Sub<Face> for Faces {
    type Output = Self;

    fn sub(self, rhs: Face) -> Self::Output {
        Self(self.0 & !(rhs as u8))
    }
}

impl SubAssign<Face> for Faces {
    fn sub_assign(&mut self, rhs: Face) {
        self.0 &= !(rhs as u8);
    }
}

// FaceIter

#[derive(Debug, Clone)]
pub struct FaceIter {
    faces: Faces,
    index: u8,
}

impl Iterator for FaceIter {
    type Item = Face;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 6 {
            self.index += 1;

            if self.faces.0 & (1 << self.index - 1) != 0 {
                return Some(Face::VALUES[self.index as usize - 1]);
            }
        }

        None
    }
}

// PerFace

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

impl<T> Index<Face> for PerFace<T> {
    type Output = T;

    fn index(&self, index: Face) -> &Self::Output {
        match index {
            Face::East => &self.east,
            Face::West => &self.west,
            Face::Up => &self.up,
            Face::Down => &self.down,
            Face::North => &self.north,
            Face::South => &self.south,
        }
    }
}

impl<T> IndexMut<Face> for PerFace<T> {
    fn index_mut(&mut self, index: Face) -> &mut Self::Output {
        match index {
            Face::East => &mut self.east,
            Face::West => &mut self.west,
            Face::Up => &mut self.up,
            Face::Down => &mut self.down,
            Face::North => &mut self.north,
            Face::South => &mut self.south,
        }
    }
}

impl<T> PerFace<T> {
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
        F: FnMut(Face) -> T,
    {
        Self {
            east: f(Face::East),
            west: f(Face::West),
            up: f(Face::Up),
            down: f(Face::Down),
            north: f(Face::North),
            south: f(Face::South),
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

// PerFaceIter

#[derive(Debug, Clone)]
pub struct PerFaceIter<'a, T> {
    value: &'a PerFace<T>,
    index: u8,
}

impl<'a, T> Iterator for PerFaceIter<'a, T> {
    type Item = (Face, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 6 {
            self.index += 1;
            Some((Face::VALUES[self.index as usize], &self.value[self.index]))
        } else {
            None
        }
    }
}

// PerFaceU5

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
