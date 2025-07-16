use std::random::random;

use lib::color::{Color, ColorConsts, Rgba};
use lib::save::{SaveAttributes, WorldAttributes, WorldDescriptor};
use lib::size::Size2;

use crate::app::{Command, Render, Update};
use crate::menu::MenuConfig;
use crate::ui::{Button, ButtonId, LayoutDirection, Ui, UiEvent};
use crate::video::ui::brush::Text;
use crate::video::ui::Painter;

#[derive(Debug)]
pub struct TitleMenu {
    ui: Ui,
    play_button_id: ButtonId,
    options_button_id: ButtonId,
    quit_button_id: ButtonId,
}

impl TitleMenu {
    pub fn new(painter: &Painter) -> Self {
        let font_id = painter.default_font_id();

        let mut play_button_id = None;
        let mut options_button_id = None;
        let mut quit_button_id = None;
        Self {
            ui: Ui::build(painter)
                .with_background_color(Rgba::from_rgb(1.0, 0.0, 0.0))
                .with_padding(Size2::new(128., 128.))
                .with_gap(16.)
                .with_layout_direction(LayoutDirection::Column)
                .with_text(Text {
                    font_id,
                    content: "Herbolution".to_string(),
                    font_size: 96.0,
                    color: Rgba::BLACK,
                })
                .with_button(
                    Button {
                        padding: Size2::splat(64.),
                        color: Rgba::BLACK,
                        text: Text {
                            font_id,
                            content: "Play".to_owned(),
                            font_size: 54.0,
                            color: Rgba::WHITE,
                        },
                    },
                    &mut play_button_id,
                )
                .with_button(
                    Button {
                        padding: Size2::splat(64.),
                        color: Rgba::BLACK,
                        text: Text {
                            font_id,
                            content: "Options".to_owned(),
                            font_size: 54.0,
                            color: Rgba::WHITE,
                        },
                    },
                    &mut options_button_id,
                )
                .with_button(
                    Button {
                        padding: Size2::splat(64.),
                        color: Rgba::BLACK,
                        text: Text {
                            font_id,
                            content: "Quit".to_owned(),
                            font_size: 54.0,
                            color: Rgba::WHITE,
                        },
                    },
                    &mut quit_button_id,
                )
                .finish(),
            play_button_id: play_button_id.unwrap(),
            options_button_id: options_button_id.unwrap(),
            quit_button_id: quit_button_id.unwrap(),
        }
    }

    pub fn update(&mut self, ctx: &mut Update) -> Option<Command> {
        let mut command = None;
        for event in self.ui.events(&ctx.input) {
            command = match event {
                &UiEvent::Clicked(id) if id == self.play_button_id => play_button_pressed(ctx),
                &UiEvent::Clicked(id) if id == self.options_button_id => Some(Command::OpenMenu(MenuConfig::Options)),
                &UiEvent::Clicked(id) if id == self.quit_button_id => Some(Command::Exit),
                _ => continue,
            }
        }

        command
    }

    pub fn render(&mut self, ctx: &mut Render) {
        self.ui.render(ctx);
    }
}

fn play_button_pressed(ctx: &Update) -> Option<Command> {
    let save = ctx
        .store
        .fs
        .create_or_open_save(
            "default",
            SaveAttributes {
                title: "Default".to_string(),
                default_world: WorldAttributes {
                    name: "world".to_string(),
                    descriptor: WorldDescriptor {
                        title: "Overworld".to_string(),
                        seed: random(),
                    },
                },
            },
        )
        .unwrap();

    Some(Command::StartGame { save })
}
