mod handler;
pub mod user_event;
pub mod async_app;

use std::marker::PhantomData;
use std::sync::Arc;
use hashbrown::HashMap;
use lazy_winit::ApplicationInit;
use thiserror::Error;
use tokio::sync::mpsc::{channel, Sender};
use tracing::{error, warn};
use winit::error::EventLoopError;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::EventLoop;
pub use winit::window::{Window, WindowId};
use winit::window::WindowAttributes;
use crate::runtime::async_app::AsyncApp;
use crate::runtime::handler::RunningRuntime;
use crate::runtime::user_event::UserEvent;

pub struct Runtime<A, U = ()> {
    window_attributes: WindowAttributes,
    _marker: PhantomData<(A, U)>,
}

#[derive(Debug, Error)]
#[error("Failed to start runtime due to event loop error: {0}")]
pub struct RuntimeError(#[from] EventLoopError);

impl<A: AsyncApp<U>, U: UserEvent> Runtime<A, U> {
    pub fn new(window_attributes: WindowAttributes) -> Self {
        Self {
            window_attributes,
            _marker: PhantomData,
        }
    }

    pub fn run(mut self) -> Result<(), RuntimeError> {
        let (wms, mut wmr) = channel(32);
        let (dms, mut dmr) = channel(32);
        let (ums, mut umr) = channel(32);
        let (ims, mut imr) = channel(32);
        let running = RunningRuntime {
            window: None,
            wms, dms, ums, ims,
            window_attributes: Some(self.window_attributes),
        };

        tokio::spawn(async move {
            let mut app;
            // Block until initialization message is received.
            if let Some(window) = imr.recv().await {
                match A::create(window).await {
                    Ok(x) => app = x,
                    Err(e) => return error!("Failed to initialize app: {e}"),
                }
            }

            loop {
                while let Ok(um) = umr.try_recv() {
                    match app.on_user_event(um) {
                        Err(e) => warn!("Failed to handle user event: {e}"),
                        _ => {}
                    }
                }

                while let Ok(dm) = dmr.try_recv() {
                    match app.on_device_event(dm.event).await {
                        Err(e) => warn!("Failed to handle device event: {e}"),
                        _ => {}
                    };
                }

                while let Ok(wm) = wmr.try_recv() {
                    match app.on_window_event(wm.event).await {
                        Err(e) => warn!("Failed to handle window event: {e}"),
                        _ => {}
                    }
                }
            }
        });

        Ok(EventLoop::new()?.run_app(&mut self)?)
    }
}

pub enum AppMessage {
    CreateWindow { name: &'static str, attributes: WindowAttributes },
}

pub struct WindowMessage {
    pub event: WindowEvent,
}

pub struct DeviceMessage {
    pub event: DeviceEvent,
}