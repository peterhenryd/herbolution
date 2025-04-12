use bitflags::bitflags;
use lazy_static::lazy_static;
use math::angle::Deg;
use math::num::traits::{ConstOne, ConstZero};
use math::rotation::{Euler, Quat};
use math::vector::{vec3i, Vec3};
use std::array::IntoIter;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Index, Range, Sub};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

lazy_static! {
    static ref ROTATIONS: [Quat; 6] = [
        Quat::from_euler(Euler::new(Deg(-90.0), Deg(0.0), Deg(0.0))),
        Quat::from_euler(Euler::new(Deg(90.0), Deg(0.0), Deg(0.0))),
        Quat::from_euler(Euler::new(Deg(0.0), Deg(-90.0), Deg(0.0))),
        Quat::from_euler(Euler::new(Deg(0.0), Deg(90.0), Deg(0.0))),
        Quat::from_euler(Euler::new(Deg(0.0), Deg(0.0), Deg(0.0))),
        Quat::from_euler(Euler::new(Deg(0.0), Deg(180.0), Deg(0.0))),
    ];
}

impl Face {
    pub fn entries() -> IntoIter<Self, 6> {
        [Face::Top, Face::Bottom, Face::Left, Face::Right, Face::Front, Face::Back].into_iter()
    }

    pub fn into_quat(self) -> Quat {
        match self {
            Face::Top => ROTATIONS[0],
            Face::Bottom => ROTATIONS[1],
            Face::Left => ROTATIONS[2],
            Face::Right => ROTATIONS[3],
            Face::Front => ROTATIONS[4],
            Face::Back => ROTATIONS[5],
        }
    }

    pub fn from_vec3(pos: vec3i) -> Option<Self> {
        match pos {
            Vec3 { x: 0, y: 1, z: 0 } => Some(Face::Top),
            Vec3 { x: 0, y: -1, z: 0 } => Some(Face::Bottom),
            Vec3 { x: -1, y: 0, z: 0 } => Some(Face::Left),
            Vec3 { x: 1, y: 0, z: 0 } => Some(Face::Right),
            Vec3 { x: 0, y: 0, z: 1 } => Some(Face::Front),
            Vec3 { x: 0, y: 0, z: -1 } => Some(Face::Back),
            _ => None,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Face::Top),
            1 => Some(Face::Bottom),
            2 => Some(Face::Left),
            3 => Some(Face::Right),
            4 => Some(Face::Front),
            5 => Some(Face::Back),
            _ => None,
        }
    }

    pub fn into_vec3(self) -> vec3i {
        match self {
            Face::Top => vec3i::new(0, 1, 0),
            Face::Bottom => vec3i::new(0, -1, 0),
            Face::Left => vec3i::new(-1, 0, 0),
            Face::Right => vec3i::new(1, 0, 0),
            Face::Front => vec3i::new(0, 0, 1),
            Face::Back => vec3i::new(0, 0, -1),
        }
    }

    pub fn inverse(self) -> Self {
        match self {
            Face::Top => Face::Bottom,
            Face::Bottom => Face::Top,
            Face::Left => Face::Right,
            Face::Right => Face::Left,
            Face::Front => Face::Back,
            Face::Back => Face::Front,
        }
    }
    
    pub fn sized_boundary_slice<T>(self, length: T) -> Vec3<Range<T>>
    where T: Copy + ConstZero + ConstOne + Sub<Output = T> {
        match self {
            Face::Top => Vec3::new(
                T::ZERO..length,
                length - T::ONE..length,
                T::ZERO..length,
            ),
            Face::Bottom => Vec3::new(
                T::ZERO..length,
                T::ZERO..T::ONE,
                T::ZERO..length,
            ),
            Face::Left => Vec3::new(
                T::ZERO..T::ONE,
                T::ZERO..length,
                T::ZERO..length,
            ),
            Face::Right => Vec3::new(
                length - T::ONE..length,
                T::ZERO..length,
                T::ZERO..length,
            ),
            Face::Front => Vec3::new(
                T::ZERO..length,
                T::ZERO..length,
                length - T::ONE..length,
            ),
            Face::Back => Vec3::new(
                T::ZERO..length,
                T::ZERO..length,
                T::ZERO..T::ONE,
            ),
        }
    }
}

impl Into<Quat> for Face {
    fn into(self) -> Quat {
        self.into_quat()
    }
}

impl From<vec3i> for Face {
    fn from(pos: vec3i) -> Self {
        Face::from_vec3(pos).unwrap()
    }
}

impl Into<vec3i> for Face {
    fn into(self) -> vec3i {
        self.into_vec3()
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Faces: u8 {
        const TOP = 0b00000001;
        const BOTTOM = 0b00000010;
        const LEFT = 0b00000100;
        const RIGHT = 0b00001000;
        const FRONT = 0b00010000;
        const BACK = 0b00100000;
    }
}

impl Faces {
    pub fn variant(self) -> Face {
        match self {
            Faces::TOP => Face::Top,
            Faces::BOTTOM => Face::Bottom,
            Faces::LEFT => Face::Left,
            Faces::RIGHT => Face::Right,
            Faces::FRONT => Face::Front,
            Faces::BACK => Face::Back,
            _ => panic!("Cannot select a variant from multiple faces"),
        }
    }
}

pub struct MapFaces<F, T> {
    faces: Faces,
    function: F,
    _marker: PhantomData<T>,
    index: u8,
}

impl<F, T> Iterator for MapFaces<F, T>
where F: Fn(Face) -> T {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 6 {
            let face = Face::from_u8(self.index).unwrap();
            self.index += 1;
            if self.faces.contains(face.into()) {
                return Some((self.function)(face));
            }
        }
        None
    }
}

impl Faces {
    pub fn map<T, F>(self, function: F) -> MapFaces<F, T>
    where F: Fn(Face) -> T {
        MapFaces {
            faces: self,
            function,
            _marker: PhantomData,
            index: 0,
        }
    }
}

impl From<Face> for Faces {
    fn from(face: Face) -> Self {
        match face {
            Face::Top => Faces::TOP,
            Face::Bottom => Faces::BOTTOM,
            Face::Left => Faces::LEFT,
            Face::Right => Faces::RIGHT,
            Face::Front => Faces::FRONT,
            Face::Back => Faces::BACK,
        }
    }
}

pub struct PerFace<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
    pub front: T,
    pub back: T,
}

impl<T> Index<usize> for PerFace<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
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
}

impl<T: Clone> Clone for PerFace<T> {
    fn clone(&self) -> Self {
        Self {
            top: self.top.clone(),
            bottom: self.bottom.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            front: self.front.clone(),
            back: self.back.clone(),
        }
    }
}

impl<T: Copy> Copy for PerFace<T> {}

impl<T: PartialEq> PartialEq for PerFace<T> {
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top
            && self.bottom == other.bottom
            && self.left == other.left
            && self.right == other.right
            && self.front == other.front
            && self.back == other.back
    }
}

impl<T: Eq> Eq for PerFace<T> {}

impl<T: Debug> Debug for PerFace<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PerFace")
            .field("top", &self.top)
            .field("bottom", &self.bottom)
            .field("left", &self.left)
            .field("right", &self.right)
            .field("front", &self.front)
            .field("back", &self.back)
            .finish()
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
