use bitflags::bitflags;
use math::angle::Deg;
use math::quat::Quat;
use math::vector::vec3i;

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