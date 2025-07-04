use std::collections::HashSet;
use std::mem::take;

use lib::vector::vec2d;
use smallvec::SmallVec;
use winit::event::Modifiers;
pub use winit::event::MouseButton;
use winit::keyboard::{KeyCode, ModifiersKeyState};

#[derive(Default)]
pub struct Input {
    active_keys: HashSet<KeyCode>,
    active_mouse_buttons: HashSet<MouseButton>,
    mouse_position: vec2d,
    frame: InputFrame,
    modifiers: Modifiers,
    is_focused: bool,
}

impl Input {
    pub fn push_key_activity(&mut self, key_code: KeyCode, is_active: bool) {
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

    pub fn push_mouse_movement(&mut self, delta: vec2d) {
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

    pub fn set_modifiers(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
    }

    pub fn is_left_control_active(&self) -> bool {
        self.modifiers.lcontrol_state() == ModifiersKeyState::Pressed
    }

    pub fn is_right_control_active(&self) -> bool {
        self.modifiers.rcontrol_state() == ModifiersKeyState::Pressed
    }

    pub fn is_left_shift_active(&self) -> bool {
        self.modifiers.lshift_state() == ModifiersKeyState::Pressed
    }

    pub fn is_right_shift_active(&self) -> bool {
        self.modifiers.rshift_state() == ModifiersKeyState::Pressed
    }

    pub fn is_left_alt_active(&self) -> bool {
        self.modifiers.lalt_state() == ModifiersKeyState::Pressed
    }

    pub fn is_right_alt_active(&self) -> bool {
        self.modifiers.ralt_state() == ModifiersKeyState::Pressed
    }

    pub fn is_left_super_active(&self) -> bool {
        self.modifiers.lsuper_state() == ModifiersKeyState::Pressed
    }

    pub fn is_right_super_active(&self) -> bool {
        self.modifiers.rsuper_state() == ModifiersKeyState::Pressed
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
}

#[derive(Debug, Default)]
pub struct InputFrame {
    pub mouse_movement: vec2d,
    pub mouse_scroll: f32,
    pub click_events: SmallVec<ClickEvent, 4>,
    pub key_events: SmallVec<KeyCode, 16>,
}

#[derive(Debug, Copy, Clone)]
pub struct ClickEvent {
    pub button: MouseButton,
    pub position: vec2d,
}
