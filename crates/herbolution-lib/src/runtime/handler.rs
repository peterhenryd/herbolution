use crate::runtime::user_event::UserEvent;
use crate::runtime::{DeviceMessage, WindowMessage};
use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::task;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

pub struct RunningRuntime<U: UserEvent> {
    pub(super) window: Option<Arc<Window>>,
    /// Window message sender.
    pub(super) wms: Sender<WindowMessage>,
    /// Device message sender.
    pub(super) dms: Sender<DeviceMessage>,
    /// User message sender.
    pub(super) ums: Sender<U::Output>,
    /// Initialization message sender.
    pub(super) ims: Sender<Arc<Window>>,
    pub(super) window_attributes: Option<WindowAttributes>,
}

impl<U: UserEvent> ApplicationHandler<U> for RunningRuntime<U> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = self.window_attributes.take().unwrap_or_default();
        self.window = Some(Arc::new(event_loop.create_window(window_attributes).unwrap()));

        let ims = self.ims.clone();
        tokio::spawn(async move {
            ims.send(self.window.clone().unwrap()).await.unwrap();
        });
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: U) {
        let ums = self.ums.clone();
        let message = event.process(self, event_loop);
        task::spawn(async move {
            ums.send(message).await.unwrap();
        });
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let wms = self.wms.clone();
        let message = WindowMessage { event };
        task::spawn(async move {
            wms.send(message).await.unwrap()
        });
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let dms = self.dms.clone();
        let message = DeviceMessage { event };
        task::spawn(async move {
            dms.send(message).await.unwrap();
        });
    }
}