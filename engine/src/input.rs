use math::vector::vec2d;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::mem::take;
pub use winit::event::MouseButton;
pub use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct Input {
    active_keys: HashSet<KeyCode>,
    active_mouse_buttons: HashSet<MouseButton>,
    mouse_position: vec2d,
    frame: InputFrame,
    is_focused: bool,
}

impl Input {
    pub fn set_key_activity(&mut self, key_code: KeyCode, is_active: bool) {
        if is_active {
            self.active_keys.insert(key_code);
            return;
        }

        if self.active_keys.contains(&key_code) {
            self.frame.key_events.push(key_code);
        }

        self.active_keys.remove(&key_code);
    }

    pub fn is_key_active(&self, key_code: KeyCode) -> bool {
        self.active_keys.contains(&key_code)
    }

    pub fn set_mouse_button_activity(&mut self, button: MouseButton, is_active: bool) {
        if is_active {
            self.active_mouse_buttons.insert(button);
            return;
        }

        if self.active_mouse_buttons.contains(&button) {
            self.frame.click_events.push(ClickEvent {
                button,
                position: self.mouse_position,
            });
        }

        self.active_mouse_buttons.remove(&button);
    }

    pub fn is_mouse_button_active(&self, button: MouseButton) -> bool {
        self.active_mouse_buttons.contains(&button)
    }

    pub fn set_mouse_position(&mut self, position: vec2d) {
        self.mouse_position = position;
    }

    pub fn add_mouse_movement(&mut self, delta: vec2d) {
        self.frame.mouse_movement += delta;
    }

    pub fn add_mouse_scroll(&mut self, delta: f32) {
        self.frame.mouse_scroll += delta;
    }

    pub fn take_frame(&mut self) -> InputFrame {
        take(&mut self.frame)
    }

    pub fn set_focused(&mut self, is_focused: bool) {
        self.is_focused = is_focused;
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
}

#[derive(Default)]
pub struct InputFrame {
    pub mouse_movement: vec2d,
    pub mouse_scroll: f32,
    pub click_events: SmallVec<[ClickEvent; 4]>,
    pub key_events: SmallVec<[KeyCode; 16]>,
}

#[derive(Debug, Copy, Clone)]
pub struct ClickEvent {
    pub button: MouseButton,
    pub position: vec2d,
}