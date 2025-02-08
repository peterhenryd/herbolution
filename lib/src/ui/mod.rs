use std::time::Duration;
use crate::engine::Engine;

pub struct Ui {

}

impl Ui {
    pub fn create(_: &Engine) -> Self {
        Self {}
    }

    pub fn update(&mut self, _: Duration) {

    }
}