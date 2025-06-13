use bitflags::bitflags;
use lazy_static::lazy_static;
use math::rotation::{Euler, Quat};
use math::vector::{vec3i, Vec3};
use std::array::IntoIter;
use std::f32::consts::{FRAC_PI_2, PI};
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use bytemuck::{Pod, Zeroable};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
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

lazy_static! {
    static ref ROTATIONS: [Quat; 6] = [
        Quat::from_euler(Euler::new(0.0, FRAC_PI_2, 0.0)),
        Quat::from_euler(Euler::new(0.0, -FRAC_PI_2, 0.0)),
        Quat::from_euler(Euler::new(-FRAC_PI_2, 0.0 , 0.0)),
        Quat::from_euler(Euler::new(FRAC_PI_2, 0.0, 0.0)),
        Quat::from_euler(Euler::new(0.0, 0.0, 0.0)),
        Quat::from_euler(Euler::new(0.0, PI, 0.0)),
    ];
}

impl Face {
    #[inline]
    pub fn entries() -> IntoIter<Self, 6> {
        [Face::East, Face::West, Face::Up, Face::Down, Face::North, Face::South].into_iter()
    }

    pub fn from_normal(pos: vec3i) -> Option<Self> {
        match pos {
            Vec3 { x: 0, y: 1, z: 0 } => Some(Face::Up),
            Vec3 { x: 0, y: -1, z: 0 } => Some(Face::Down),
            Vec3 { x: -1, y: 0, z: 0 } => Some(Face::West),
            Vec3 { x: 1, y: 0, z: 0 } => Some(Face::East),
            Vec3 { x: 0, y: 0, z: 1 } => Some(Face::North),
            Vec3 { x: 0, y: 0, z: -1 } => Some(Face::South),
            _ => None,
        }
    }

    pub fn from_index(value: u8) -> Option<Self> {
        match value {
            0 => Some(Face::East),
            1 => Some(Face::West),
            2 => Some(Face::Up),
            3 => Some(Face::Down),
            4 => Some(Face::North),
            5 => Some(Face::South),
            _ => None,
        }
    }

    pub fn to_rotation(self) -> Quat {
        match self {
            Face::East => ROTATIONS[0],
            Face::West => ROTATIONS[1],
            Face::Up => ROTATIONS[2],
            Face::Down => ROTATIONS[3],
            Face::North => ROTATIONS[4],
            Face::South => ROTATIONS[5],
        }
    }

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

    #[inline]
    pub fn to_index(self) -> u8 {
        self as u8
    }

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

impl Into<Quat> for Face {
    fn into(self) -> Quat {
        self.to_rotation()
    }
}

impl From<vec3i> for Face {
    fn from(pos: vec3i) -> Self {
        Face::from_normal(pos).unwrap()
    }
}

impl Into<vec3i> for Face {
    fn into(self) -> vec3i {
        self.to_normal()
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Faces: u8 {
        const EAST = 1 << 0;
        const WEST = 1 << 1;
        const UP = 1 << 2;
        const DOWN = 1 << 3;
        const NORTH = 1 << 4;
        const SOUTH = 1 << 5;
    }
}

impl Faces {
    pub fn to_variant(self) -> Option<Face> {
        match self {
            Faces::EAST => Some(Face::East),
            Faces::WEST => Some(Face::West),
            Faces::UP => Some(Face::Up),
            Faces::DOWN => Some(Face::Down),
            Faces::NORTH => Some(Face::North),
            Faces::SOUTH => Some(Face::South),
            _ => None,
        }
    }

    pub fn variant_iter(self) -> VariantIter {
        VariantIter {
            faces: self.bits(),
            index: 0,
        }
    }
}

impl From<Face> for Faces {
    fn from(face: Face) -> Self {
        match face {
            Face::East => Faces::EAST,
            Face::West => Faces::WEST,
            Face::Up => Faces::UP,
            Face::Down => Faces::DOWN,
            Face::North => Faces::NORTH,
            Face::South => Faces::SOUTH,
        }
    }
}

pub struct VariantIter {
    faces: u8,
    index: u8,
}

impl Iterator for VariantIter {
    type Item = Face;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 6 {
            let face = Face::from_index(self.index)?;
            if self.faces & (1 << self.index) != 0 {
                self.index += 1;
                return Some(face);
            }
            self.index += 1;
        }
        None
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PerFace<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
    pub front: T,
    pub back: T,
}

impl<T> Index<u8> for PerFace<T> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.top,
            1 => &self.bottom,
            2 => &self.left,
            3 => &self.right,
            4 => &self.front,
            5 => &self.back,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<u8> for PerFace<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.top,
            1 => &mut self.bottom,
            2 => &mut self.left,
            3 => &mut self.right,
            4 => &mut self.front,
            5 => &mut self.back,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> Index<Face> for PerFace<T> {
    type Output = T;

    fn index(&self, index: Face) -> &Self::Output {
        match index {
            Face::Up => &self.top,
            Face::Down => &self.bottom,
            Face::West => &self.left,
            Face::East => &self.right,
            Face::North => &self.front,
            Face::South => &self.back,
        }
    }
}

impl<T> IndexMut<Face> for PerFace<T> {
    fn index_mut(&mut self, index: Face) -> &mut Self::Output {
        match index {
            Face::Up => &mut self.top,
            Face::Down => &mut self.bottom,
            Face::West => &mut self.left,
            Face::East => &mut self.right,
            Face::North => &mut self.front,
            Face::South => &mut self.back,
        }
    }
}

impl<T> PerFace<T> {
    pub const fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
            front: value,
            back: value,
        }
    }

    pub fn mapped<F>(f: F) -> Self
    where
        F: Fn(Face) -> T,
    {
        Self {
            top: f(Face::Up),
            bottom: f(Face::Down),
            left: f(Face::West),
            right: f(Face::East),
            front: f(Face::North),
            back: f(Face::South),
        }
    }

    pub fn as_array(&self) -> &[T; 6]
    where 
        T: Pod, 
    {
        bytemuck::cast_ref(self)
    }

    pub fn iter(&self) -> PerFaceIter<T> {
        PerFaceIter {
            value: self,
            index: 0,
        }
    }
}

impl<T: Default> Default for PerFace<T> {
    fn default() -> Self {
        Self {
            top: T::default(),
            bottom: T::default(),
            left: T::default(),
            right: T::default(),
            front: T::default(),
            back: T::default(),
        }
    }
}

unsafe impl<T: Zeroable> Zeroable for PerFace<T> {}

unsafe impl<T: Pod> Pod for PerFace<T> {}

pub struct PerFaceIter<'a, T> {
    value: &'a PerFace<T>,
    index: u8,
}

impl<'a, T> Iterator for PerFaceIter<'a, T> {
    type Item = (Face, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 6 {
            self.index += 1;
            Some((Face::from_index(self.index).unwrap(), &self.value[self.index]))
        } else {
            None
        }
    }
}
