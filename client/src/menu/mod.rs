use crate::app::{Command, Render, Update};
use crate::menu::options::OptionsMenu;
use crate::menu::play::PlayMenu;
use crate::menu::title::TitleMenu;
use crate::video::ui::Painter;

pub mod config;
pub mod options;
pub mod play;
pub mod title;

/// The active menu state of the application.
#[derive(Debug)]
pub enum Menu {
    Title(TitleMenu),
    Options(OptionsMenu),
    Play(PlayMenu),
}

impl Menu {
    pub fn new(config: MenuConfig, painter: &Painter) -> Self {
        match config {
            MenuConfig::Title => Menu::Title(TitleMenu::new(painter)),
            MenuConfig::Options => Menu::Options(OptionsMenu::new()),
            MenuConfig::Play => Menu::Play(PlayMenu::new()),
        }
    }

    /// Updates the active menu state using the provided context.
    pub fn update(&mut self, context: &mut Update) -> Option<Command> {
        match self {
            Menu::Title(x) => x.update(context),
            Menu::Options(x) => x.update(context),
            Menu::Play(x) => x.update(context),
        }
    }

    /// Renders the active menu using the provided context.
    pub fn render<'t>(&'t mut self, ctx: &mut Render) {
        match self {
            Menu::Title(x) => x.render(ctx),
            Menu::Options(x) => x.render(ctx),
            Menu::Play(x) => x.render(ctx),
        }
    }
}

impl From<TitleMenu> for Menu {
    fn from(menu: TitleMenu) -> Self {
        Menu::Title(menu)
    }
}

impl From<OptionsMenu> for Menu {
    fn from(menu: OptionsMenu) -> Self {
        Menu::Options(menu)
    }
}

impl From<PlayMenu> for Menu {
    fn from(menu: PlayMenu) -> Self {
        Menu::Play(menu)
    }
}

/// A menu configuration used to construct a given menu with the provided options.
#[derive(Debug, Clone)]
pub enum MenuConfig {
    Title,
    Options,
    Play,
}
