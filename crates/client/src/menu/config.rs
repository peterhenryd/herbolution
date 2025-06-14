use crate::menu::options::OptionsMenu;
use crate::menu::play::PlayMenu;
use crate::menu::title::TitleMenu;
use crate::menu::Menu;

/// A menu configuration used to construct a given menu with the provided options.
#[derive(Debug, Clone)]
pub enum MenuConfig {
    Title,
    Options,
    Play,
}

impl Into<Menu> for MenuConfig {
    fn into(self) -> Menu {
        match self {
            MenuConfig::Title => TitleMenu::new().into(),
            MenuConfig::Options => OptionsMenu::new().into(),
            MenuConfig::Play => PlayMenu::new().into(),
        }
    }
}
