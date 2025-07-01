use crate::app::state::Command;
use crate::app::{Render, Update};
use crate::menu::options::OptionsMenu;
use crate::menu::play::PlayMenu;
use crate::menu::title::TitleMenu;

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
        let mut brush = ctx.frame.draw_2d();

        match self {
            Menu::Title(x) => x.render(&mut brush),
            Menu::Options(x) => x.render(&mut brush),
            Menu::Play(x) => x.render(&mut brush),
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
