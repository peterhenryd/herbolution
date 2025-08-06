use std::random::random;

use kiln::{FontId, Text};
use lib::save::{SaveAttributes, WorldAttributes, WorldDescriptor};
use prims::color::{ColorConsts, Rgba};
use prims::size::Size2;

use crate::app::state::Command;
use crate::app::{Render, Update};
use crate::menu::MenuConfig;
use crate::ui::{Button, ButtonId, LayoutDirection, Ui, UiEvent};

#[derive(Debug)]
pub struct TitleMenu {
    ui: Ui,
    play_button_id: ButtonId,
    options_button_id: ButtonId,
    quit_button_id: ButtonId,
}

impl TitleMenu {
    pub fn new() -> Self {
        let mut play_button_id = None;
        let mut options_button_id = None;
        let mut quit_button_id = None;
        Self {
            ui: Ui::build()
                .with_padding(Size2::new(128., 128.))
                .with_gap(16.)
                .with_layout_direction(LayoutDirection::Column)
                .with_text(Text {
                    font_id: FontId::default(),
                    content: ('a'..='z')
                        .chain('A'..='Z')
                        .chain('0'..'9')
                        .collect(),
                    font_size: 96.0,
                    color: Rgba::BLACK,
                })
                .with_button(
                    Button {
                        padding: Size2::splat(64.),
                        color: Rgba::BLACK,
                        text: Text {
                            font_id: FontId::default(),
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
                            font_id: FontId::default(),
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
                            font_id: FontId::default(),
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
        let mut _command = None;
        for event in self.ui.events(&ctx.input) {
            _command = match event {
                &UiEvent::Clicked(id) if id == self.play_button_id => play_button_pressed(ctx),
                &UiEvent::Clicked(id) if id == self.options_button_id => Some(Command::OpenMenu(MenuConfig::Options)),
                &UiEvent::Clicked(id) if id == self.quit_button_id => Some(Command::Exit),
                _ => continue,
            }
        }

        Some(Command::StartGame {
            save: ctx
                .app
                .fs
                .create_or_open_save(
                    "default",
                    SaveAttributes {
                        title: "Default".to_string(),
                        default_world: WorldAttributes {
                            name: "world".to_string(),
                            descriptor: WorldDescriptor {
                                title: "World".to_string(),
                                seed: random(..),
                            },
                        },
                    },
                )
                .unwrap(),
        })
    }

    pub fn render(&mut self, ctx: &mut Render) {
        ctx.frame
            .push_clear_color(Rgba::new(1.0, 0.0, 0.0, 1.0));
        self.ui.render(
            &mut ctx
                .frame
                .prepare_2d(ctx.app.core2d)
                .draw_ui(ctx.app.core2d),
        );
    }
}

fn play_button_pressed(ctx: &Update) -> Option<Command> {
    let save = ctx
        .app
        .fs
        .create_or_open_save(
            "default",
            SaveAttributes {
                title: "Default".to_string(),
                default_world: WorldAttributes {
                    name: "world".to_string(),
                    descriptor: WorldDescriptor {
                        title: "Overworld".to_string(),
                        seed: random(..),
                    },
                },
            },
        )
        .unwrap();

    Some(Command::StartGame { save })
}
