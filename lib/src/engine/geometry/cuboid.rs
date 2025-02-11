use bitflags::bitflags;
use math::angle::Deg;
use math::quat::Quat;
use math::vector::{vec3, vec3f, vec3i};
use num::traits::real::Real;
use num::traits::ConstZero;
use num::{Float, Num};
use std::fmt::Debug;
use std::ops::{Add, Index};

/// A cuboid in 3D space, also used as an axis-aligned bounding box.
pub struct Cuboid<T> {
    pub min: vec3<T>,
    pub max: vec3<T>,
}

impl<T> Cuboid<T> {
    pub const fn new(min: vec3<T>, max: vec3<T>) -> Self {
        Self { min, max }
    }

    pub fn union(slice: &[Self]) -> Self
    where
        T: Copy + Real + ConstZero,
    {
        slice.iter()
            .copied()
            .reduce(|lhs, rhs| Self {
                min: lhs.min.min(rhs.min),
                max: lhs.max.max(rhs.max),
            })
            .unwrap_or(Cuboid::ZERO)
    }

    #[inline]
    pub fn width(&self) -> T
    where
        T: Copy + Num,
    {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> T
    where
        T: Copy + Num,
    {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn depth(&self) -> T
    where
        T: Copy + Num,
    {
        self.max.z - self.min.z
    }

    #[inline]
    pub fn length_squared(&self) -> T
    where
        T: Copy + Float,
    {
        self.width().powi(2) + self.height().powi(2) + self.depth().powi(2)
    }

    #[inline]
    pub fn length(&self) -> T
    where
        T: Copy + Float,
    {
        self.length_squared().sqrt()
    }

    pub fn set_position(&mut self, position: vec3<T>)
    where
        T: Copy + Num,
    {
        self.max.x = position.x + self.width();
        self.max.y = position.y + self.height();
        self.max.z = position.z + self.depth();
        self.min = position;
    }

    pub fn intersect(&self, other: &Cuboid<T>) -> Self
    where
        T: Copy + Num,
    {
        Self {
            min: self.min - other.min,
            max: self.max - other.max,
        }
    }

    pub fn intersects(&self, other: &Cuboid<T>) -> bool
    where
        T: Copy + PartialOrd,
    {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    pub fn is_touching(&self, other: &Cuboid<T>) -> bool
    where
        T: Copy + Num,
    {
        let intersection = self.intersect(other);
        intersection.width() == T::zero() || intersection.height() == T::zero() || intersection.depth() == T::zero()
    }
}

impl<T: ConstZero> Cuboid<T> {
    pub const ZERO: Self = Self {
        min: vec3::ZERO,
        max: vec3::ZERO,
    };
}

impl<T: Copy> Copy for Cuboid<T> {}

impl<T: Clone> Clone for Cuboid<T> {
    fn clone(&self) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl<T: Debug> Debug for Cuboid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cuboid")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl From<Faces> for Cuboid<f32> {
    fn from(faces: Faces) -> Self {
        let mut min = vec3::splat(f32::MAX);
        let mut max = vec3::splat(f32::MIN);

        for corner in faces.map(Face::into_corners).into_flattened() {
            min = min.min(corner);
            max = max.max(corner);
        }

        Self { min, max }
    }
}

impl<T: Copy + Add<Output=T>> Add<vec3<T>> for Cuboid<T> {
    type Output = Self;

    fn add(self, rhs: vec3<T>) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

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

impl Face {
    pub fn into_quat(self) -> Quat {
        match self {
            Face::Top => Quat::from_euler(Deg(-90.0), Deg(0.0), Deg(0.0)),
            Face::Bottom => Quat::from_euler(Deg(90.0), Deg(0.0), Deg(0.0)),
            Face::Left => Quat::from_euler(Deg(0.0), Deg(-90.0), Deg(0.0)),
            Face::Right => Quat::from_euler(Deg(0.0), Deg(90.0), Deg(0.0)),
            Face::Front => Quat::from_euler(Deg(0.0), Deg(0.0), Deg(0.0)),
            Face::Back => Quat::from_euler(Deg(0.0), Deg(180.0), Deg(0.0)),
        }
    }

    pub fn from_vec3i(pos: vec3i) -> Option<Self> {
        match pos {
            vec3 { x: 0, y: 1, z: 0 } => Some(Face::Top),
            vec3 { x: 0, y: -1, z: 0 } => Some(Face::Bottom),
            vec3 { x: -1, y: 0, z: 0 } => Some(Face::Left),
            vec3 { x: 1, y: 0, z: 0 } => Some(Face::Right),
            vec3 { x: 0, y: 0, z: 1 } => Some(Face::Front),
            vec3 { x: 0, y: 0, z: -1 } => Some(Face::Back),
            _ => None,
        }
    }

    pub fn into_vec3i(self) -> vec3i {
        match self {
            Face::Top => vec3i::new(0, 1, 0),
            Face::Bottom => vec3i::new(0, -1, 0),
            Face::Left => vec3i::new(-1, 0, 0),
            Face::Right => vec3i::new(1, 0, 0),
            Face::Front => vec3i::new(0, 0, 1),
            Face::Back => vec3i::new(0, 0, -1),
        }
    }

    pub fn into_corners(self) -> [vec3f; 4] {
        match self {
            Face::Top => [
                vec3::new(0., 1., 0.),
                vec3::new(0., 1., 1.),
                vec3::new(1., 1., 1.),
                vec3::new(1., 1., 0.),
            ],
            Face::Bottom => [
                vec3::new(0., 0., 0.),
                vec3::new(0., 0., 1.),
                vec3::new(1., 0., 1.),
                vec3::new(1., 0., 0.),
            ],
            Face::Left => [
                vec3::new(0., 0., 0.),
                vec3::new(0., 0., 1.),
                vec3::new(0., 1., 1.),
                vec3::new(0., 1., 0.),
            ],
            Face::Right => [
                vec3::new(1., 0., 0.),
                vec3::new(1., 0., 1.),
                vec3::new(1., 1., 1.),
                vec3::new(1., 1., 0.),
            ],
            Face::Front => [
                vec3::new(0., 0., 1.),
                vec3::new(0., 1., 1.),
                vec3::new(1., 1., 1.),
                vec3::new(1., 0., 1.),
            ],
            Face::Back => [
                vec3::new(0., 0., 0.),
                vec3::new(0., 1., 0.),
                vec3::new(1., 1., 0.),
                vec3::new(1., 0., 0.),
            ],
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
}

impl Into<Quat> for Face {
    fn into(self) -> Quat {
        self.into_quat()
    }
}

impl From<vec3i> for Face {
    fn from(pos: vec3i) -> Self {
        Face::from_vec3i(pos).unwrap()
    }
}

impl Into<vec3i> for Face {
    fn into(self) -> vec3i {
        self.into_vec3i()
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
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
    pub fn map<T>(self, f: impl Fn(Face) -> T) -> Vec<T> {
        let mut values = Vec::with_capacity(6);
        if self.contains(Faces::TOP) { values.push(f(Face::Top)); }
        if self.contains(Faces::BOTTOM) { values.push(f(Face::Bottom)); }
        if self.contains(Faces::LEFT) { values.push(f(Face::Left)); }
        if self.contains(Faces::RIGHT) { values.push(f(Face::Right)); }
        if self.contains(Faces::FRONT) { values.push(f(Face::Front)); }
        if self.contains(Faces::BACK) { values.push(f(Face::Back)); }
        values
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