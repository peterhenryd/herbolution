use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct Input {
    pub mouse: MouseInput,
    active_keys: Vec<KeyCode>,
    action_position: Option<PhysicalPosition<f64>>,
    pub(crate) mouse_delta: Option<(f64, f64)>,
}

impl Input {
    pub fn update(&mut self, input: InputEvent) {
        use ElementState::{Pressed, Released};
        // TODO: add soft coding of key bindings
        match input {
            /*
            InputEvent::Type {
                key: KeyCode::KeyW,
                state: element,
            } if let Some(game) = &mut state.game => {
                game.send_input_message(InputMessage::Keyed {
                    action: KeyedAction::MoveForward,
                    state: element,
                });
            }
            InputEvent::Type {
                key: KeyCode::KeyA,
                state: element,
            } if let Some(game) = &mut state.game => {
                game.send_input_message(InputMessage::Keyed {
                    action: KeyedAction::MoveLeft,
                    state: element,
                });
            }
            InputEvent::Type {
                key: KeyCode::KeyS,
                state: element,
            } if let Some(game) = &mut state.game => {
                game.send_input_message(InputMessage::Keyed {
                    action: KeyedAction::MoveBackward,
                    state: element,
                });
            }
            InputEvent::Type {
                key: KeyCode::KeyD,
                state: element,
            } if let Some(game) = &mut state.game => {
                game.send_input_message(InputMessage::Keyed {
                    action: KeyedAction::MoveRight,
                    state: element,
                });
            }
            InputEvent::Type {
                key: KeyCode::ShiftLeft,
                state: element,
            } if let Some(game) = &mut state.game => {
                game.send_input_message(InputMessage::Keyed {
                    action: KeyedAction::MoveDown,
                    state: element,
                });
            }
            InputEvent::Type {
                key: KeyCode::Space,
                state: element,
            } if let Some(game) = &mut state.game => {
                game.send_input_message(InputMessage::Keyed {
                    action: KeyedAction::MoveUp,
                    state: element,
                });
            }
            InputEvent::Move { dx, dy } if let Some(game) = &mut state.game => {
                game.send_input_message(InputMessage::MouseMoved { dx, dy });
            }

             */
            InputEvent::Type {
                key,
                state: Pressed,
            } => {
                self.add_key_down(key);
            }
            InputEvent::Type {
                key,
                state: Released,
            } => self.remove_key_down(key),
            InputEvent::Click {
                button,
                state,
                x,
                y,
            } => self.set_mouse_state(button, state == Pressed, (x, y).into()),
            InputEvent::Move { dx, dy } => {
                self.mouse_delta = Some((dx, dy));
            }
        }
    }

    pub fn is_any_key_down(&self) -> bool {
        !self.active_keys.is_empty()
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.active_keys.contains(&key)
    }

    pub fn add_key_down(&mut self, key: KeyCode) {
        self.active_keys.push(key);
    }

    pub fn remove_key_down(&mut self, key: KeyCode) {
        self.active_keys.retain(|&k| k != key);
    }

    pub fn set_mouse_state(
        &mut self,
        button: MouseButton,
        is_down: bool,
        position: PhysicalPosition<f64>,
    ) {
        match button {
            MouseButton::Left => self.mouse.is_left_down = is_down,
            MouseButton::Right => self.mouse.is_right_down = is_down,
            MouseButton::Middle => self.mouse.is_middle_down = is_down,
            _ => {}
        }
        self.action_position = Some(position);
    }

    pub fn consume_position(&mut self) -> Option<PhysicalPosition<f64>> {
        self.action_position.take()
    }

    pub fn consume_mouse_delta(&mut self) -> Option<(f64, f64)> {
        self.mouse_delta.take()
    }
}

#[derive(Default)]
pub struct MouseInput {
    pub is_left_down: bool,
    pub is_right_down: bool,
    pub is_middle_down: bool,
}

pub enum InputEvent {
    Type {
        key: KeyCode,
        state: ElementState,
    },
    Move {
        dx: f64,
        dy: f64,
    },
    Click {
        button: MouseButton,
        state: ElementState,
        x: f64,
        y: f64,
    },
}