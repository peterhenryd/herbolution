use crate::engine::Engine;
use crate::game::Game;
use crate::menu::{MenuProvider, MenuType, NextState};
use wgpu::{
    Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp,
    TextureView, TextureViewDescriptor,
};

pub struct State {
    menu_provider: MenuProvider,
    pub(crate) game: Option<Game>,
}

impl State {
    pub fn new() -> Self {
        Self {
            menu_provider: MenuProvider::new(),
            game: None,
        }
    }

    pub fn start(&mut self, engine: &mut Engine) {
        self.menu_provider.set_active_menu(engine, MenuType::Title);
    }

    pub fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time.next();

        if let Some(ref mut game) = self.game {
            engine.world.update();
            game.update_world(&mut engine.world);
        }

        let menu = self.menu_provider.get_active_menu_mut();
        if let Some(state) = menu.update(dt, engine) {
            match state {
                NextState::Menu(menu_type) => self.menu_provider.set_active_menu(engine, menu_type),
                NextState::Game(address) => {
                    self.menu_provider.set_active_menu(engine, None);
                    self.game = Some(Game::connect(address));
                }
            }
        }
    }

    pub fn render(&self, engine: &mut Engine) {
        engine.ui.text.prepare();

        let surface_texture = engine
            .surface
            .get_texture()
            .expect("Failed to acquire surface texture");
        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut encoder = engine.gpu.create_command_encoder("renderer");

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("herbolution_primary_render_pass"),
            color_attachments: &clear_attachments(&texture_view),
            depth_stencil_attachment: if self.game.is_some() {
                Some(engine.surface.depth_stencil_attachment())
            } else {
                None
            },
            ..Default::default()
        });

        if self.game.is_some() {
            engine.world.render(&mut render_pass);

            drop(render_pass);
            render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("herbolution_secondary_render_pass"),
                color_attachments: &load_attachments(&texture_view),
                ..Default::default()
            });
        }

        self.menu_provider
            .get_active_menu()
            .render(engine, &mut render_pass);

        drop(render_pass);

        engine.gpu.queue.submit(Some(encoder.finish()));
        surface_texture.present();

        engine.ui.text.cleanup();
    }

    pub fn exit(&self) {
        let Some(game) = &self.game else { return };
        match game {
            Game::Local(x) => x.exit(),
            Game::Remote(_) => {}
        }
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
