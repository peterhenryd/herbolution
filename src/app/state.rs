use crate::app::gpu::Gpu;
use crate::app::surface::Surface;
use crate::game::Game;
use crate::menu::{MenuProvider, MenuType};
use crate::ui::Ui;
use crate::world::World;
use std::time::Duration;
use wgpu::{Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureView, TextureViewDescriptor};

pub struct State {
    menu_provider: MenuProvider,
    active_menu: Option<MenuType>,
    game: Option<Game>,
}

impl State {
    pub fn new(active_menu: MenuType) -> Self {
        Self {
            menu_provider: MenuProvider::new(active_menu),
            active_menu: None,
            game: None,
        }
    }

    pub fn update(&mut self, dt: Duration, ui: &mut Ui, world: &mut World) {
        if let Some(ref mut game) = self.game {
            game.update(world);
        }

        let active_menu = self.menu_provider.active_menu;
        let has_changed = self.active_menu != active_menu;
        if has_changed && self.active_menu.is_some() {
            self.menu_provider.get_menu_mut(self.active_menu.unwrap()).hide(ui);
        }
        let menu = self.menu_provider.get_active_menu_mut();
        if has_changed {
            menu.show(ui);
            self.active_menu = active_menu;
        }

        if let Some(menu_type) = menu.update(dt, ui) {
            self.menu_provider.set_active_menu(menu_type);
        }
    }

    pub fn render(&self, gpu: &Gpu, surface: &Surface, ui: &mut Ui, world: &World) {
        ui.text.prepare();

        let surface_texture = surface.get_texture().expect("Failed to acquire surface texture");
        let texture_view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = gpu.create_command_encoder("renderer");

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("herbolution_primary_render_pass"),
            color_attachments: &clear_attachments(&texture_view),
            ..Default::default()
        });

        if self.game.is_some() {
            world.render(&mut render_pass);

            drop(render_pass);
            render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("herbolution_secondary_render_pass"),
                color_attachments: &load_attachments(&texture_view),
                ..Default::default()
            });
        }

        self.menu_provider.get_active_menu().render(ui, &mut render_pass);

        drop(render_pass);

        gpu.queue.submit(Some(encoder.finish()));
        surface_texture.present();

        ui.text.cleanup();
    }
}

fn clear_attachments(view: &TextureView) -> [Option<RenderPassColorAttachment>; 1] {
    [Some(RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: Operations {
            load: LoadOp::Clear(Color::BLACK),
            store: StoreOp::Store,
        },
    })]
}

fn load_attachments(view: &TextureView) -> [Option<RenderPassColorAttachment>; 1] {
    [Some(RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: Operations {
            load: LoadOp::Load,
            store: StoreOp::Store,
        },
    })]
}