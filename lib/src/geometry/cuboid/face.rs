use std::fmt::Debug;
use std::ops::Index;
use bitflags::bitflags;
use lazy_static::lazy_static;
use math::angle::Deg;
use math::rotation::{Euler, Quat};
use math::vector::{vec3f, vec3i, Vec3};

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

    pub fn from_vec3i(pos: vec3i) -> Option<Self> {
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
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 1.),
                Vec3::new(1., 1., 1.),
                Vec3::new(1., 1., 0.),
            ],
            Face::Bottom => [
                Vec3::new(0., 0., 0.),
                Vec3::new(0., 0., 1.),
                Vec3::new(1., 0., 1.),
                Vec3::new(1., 0., 0.),
            ],
            Face::Left => [
                Vec3::new(0., 0., 0.),
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 1., 1.),
                Vec3::new(0., 1., 0.),
            ],
            Face::Right => [
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 0., 1.),
                Vec3::new(1., 1., 1.),
                Vec3::new(1., 1., 0.),
            ],
            Face::Front => [
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 1., 1.),
                Vec3::new(1., 1., 1.),
                Vec3::new(1., 0., 1.),
            ],
            Face::Back => [
                Vec3::new(0., 0., 0.),
                Vec3::new(0., 1., 0.),
                Vec3::new(1., 1., 0.),
                Vec3::new(1., 0., 0.),
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
        if self.contains(Faces::TOP) {
            values.push(f(Face::Top));
        }
        if self.contains(Faces::BOTTOM) {
            values.push(f(Face::Bottom));
        }
        if self.contains(Faces::LEFT) {
            values.push(f(Face::Left));
        }
        if self.contains(Faces::RIGHT) {
            values.push(f(Face::Right));
        }
        if self.contains(Faces::FRONT) {
            values.push(f(Face::Front));
        }
        if self.contains(Faces::BACK) {
            values.push(f(Face::Back));
        }
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
