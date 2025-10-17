use crate::app::{Command, Render, Update};

#[derive(Debug)]
pub struct PlayMenu {}

impl PlayMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _: &mut Update) -> Option<Command> {
        None
    }

    pub fn render(&mut self, _: &mut Render) {}
}
