use crate::engine::Engine;
use crate::game::address::GameAddress;
use crate::menu::title_menu::TitleMenu;
use crate::util::mem_prev::MemPrev;
use std::time::Duration;
use wgpu::RenderPass;

mod title_menu;

pub struct MenuProvider {
    pub(crate) active_menu: MemPrev<Option<MenuType>>,
    no_op_menu: NoOpMenu,
    title_menu: TitleMenu,
}

impl MenuProvider {
    pub fn new() -> Self {
        Self {
            active_menu: MemPrev::new(None),
            no_op_menu: NoOpMenu,
            title_menu: TitleMenu::new(),
        }
    }

    pub fn set_active_menu(
        &mut self,
        engine: &mut Engine,
        active_menu: impl Into<Option<MenuType>>,
    ) {
        self.active_menu.set(active_menu.into());

        if let Some(&Some(menu_type)) = self.active_menu.get_prev() {
            self.get_menu_mut(menu_type).hide(engine);
        }
        self.get_active_menu_mut().show(engine);
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
        match *self.active_menu.get() {
            None => &self.no_op_menu,
            Some(x) => self.get_menu(x),
        }
    }

    pub fn get_active_menu_mut(&mut self) -> &mut dyn Menu {
        match *self.active_menu.get() {
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
    fn show(&mut self, engine: &mut Engine);

    fn update(&mut self, dt: Duration, engine: &mut Engine) -> Option<NextState>;

    fn hide(&mut self, engine: &mut Engine);

    fn render(&self, engine: &Engine, render_pass: &mut RenderPass<'_>);
}

/// A menu that does nothing. This is used when no menu is active.
struct NoOpMenu;

impl Menu for NoOpMenu {
    fn show(&mut self, _: &mut Engine) {}

    fn update(&mut self, _: Duration, _: &mut Engine) -> Option<NextState> {
        None
    }

    fn hide(&mut self, _: &mut Engine) {}

    fn render(&self, _: &Engine, _: &mut RenderPass<'_>) {}
}

pub enum NextState {
    Menu(MenuType),
    Game(GameAddress),
}
