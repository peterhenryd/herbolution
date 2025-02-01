use std::time::Duration;
use wgpu::RenderPass;
use crate::menu::title_menu::TitleMenu;
use crate::ui::Ui;

mod title_menu;

pub struct MenuProvider {
    pub(crate) active_menu: Option<MenuType>,
    no_op_menu: NoOpMenu,
    title_menu: TitleMenu,
}

impl MenuProvider {
    pub fn new(active_menu: impl Into<Option<MenuType>>) -> Self {
        Self {
            active_menu: active_menu.into(),
            no_op_menu: NoOpMenu,
            title_menu: TitleMenu::new(),
        }
    }

    pub fn set_active_menu(&mut self, active_menu: impl Into<Option<MenuType>>) {
        self.active_menu = active_menu.into();
    }

    pub fn get_menu(&self, menu_type: MenuType) -> &dyn Menu {
        match menu_type {
            MenuType::Title => &self.title_menu,
        }
    }

    pub fn get_menu_mut(&mut self, menu_type: MenuType) -> &mut dyn Menu {
        match menu_type {
            MenuType::Title => &mut self.title_menu,
        }
    }

    pub fn get_active_menu(&self) -> &dyn Menu {
        match self.active_menu {
            None => &self.no_op_menu,
            Some(x) => self.get_menu(x),
        }
    }

    pub fn get_active_menu_mut(&mut self) -> &mut dyn Menu {
        match self.active_menu {
            None => &mut self.no_op_menu,
            Some(x) => self.get_menu_mut(x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MenuType {
    Title,
}

pub trait Menu {
    fn show(&mut self, ui: &mut Ui);

    fn update(&mut self, dt: Duration, ui: &mut Ui) -> Option<MenuType>;

    fn hide(&mut self, ui: &mut Ui);

    fn render(&self, ui: &Ui, render_pass: &mut RenderPass<'_>);
}

/// A menu that does nothing. This is used when no menu is active.
struct NoOpMenu;

impl Menu for NoOpMenu {
    fn show(&mut self, _: &mut Ui) {}

    fn update(&mut self, _: Duration, _: &mut Ui) -> Option<MenuType> { None }

    fn hide(&mut self, _: &mut Ui) {}

    fn render(&self, _: &Ui, _: &mut RenderPass<'_>) {}
}