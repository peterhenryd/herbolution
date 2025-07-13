use crate::app::{Command, Render, Update};
use crate::menu::options::OptionsMenu;
use crate::menu::play::PlayMenu;
use crate::menu::title::TitleMenu;
use crate::video::ui::Painter;

pub mod options;
pub mod play;
pub mod title;

#[derive(Debug)]
pub enum Menu {
    Title(TitleMenu),
    Options(OptionsMenu),
    Play(PlayMenu),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MenuConfig {
    Title,
    Options,
    Play,
}

impl Menu {
    pub fn new(config: MenuConfig, painter: &Painter) -> Self {
        match config {
            MenuConfig::Title => Menu::Title(TitleMenu::new(painter)),
            MenuConfig::Options => Menu::Options(OptionsMenu::new()),
            MenuConfig::Play => Menu::Play(PlayMenu::new()),
        }
    }

    pub fn update(&mut self, ctx: &mut Update) -> Option<Command> {
        match self {
            Menu::Title(x) => x.update(ctx),
            Menu::Options(x) => x.update(ctx),
            Menu::Play(x) => x.update(ctx),
        }
    }

    pub fn render<'t>(&'t mut self, ctx: &mut Render) {
        match self {
            Menu::Title(x) => x.render(ctx),
            Menu::Options(x) => x.render(ctx),
            Menu::Play(x) => x.render(ctx),
        }
    }
}
