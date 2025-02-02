use crate::chunk::material::Material;
use crate::chunk::{Chunk, ChunkLocalPosition};
use herbolution_math::vector::{vec3f, vec3i};
use winit::event::{ElementState, MouseButton};

pub enum InputMessage {
    MouseClicked {
        button: MouseButton,
        state: ElementState,
    },
    MouseMoved {
        dx: f64,
        dy: f64,
    },
    Keyed {
        action: KeyedAction,
        state: ElementState,
    },
}

pub enum KeyedAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    // Moving the camera up or jumping.
    MoveUp,
    // Moving the camera down or crouching.
    MoveDown,
}

pub enum GameMessage {
    CubeRemoved {
        position: ChunkLocalPosition,
    },
    CubeAdded {
        position: ChunkLocalPosition,
        material: Material,
    },
    ChunkLoaded {
        chunk: Box<Chunk>,
    },
    ChunkUnloaded {
        position: vec3i,
    },
    MovePlayer {
        velocity: vec3f,
    },
    RotatePlayer {
        rotation: (f64, f64),
    },
    Exit,
}
