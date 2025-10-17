use crate::app::{Command, Render, Update};

#[derive(Debug)]
pub struct OptionsMenu {}

impl OptionsMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _: &mut Update) -> Option<Command> {
        None
    }

    pub fn render(&mut self, _: &mut Render) {}
}
