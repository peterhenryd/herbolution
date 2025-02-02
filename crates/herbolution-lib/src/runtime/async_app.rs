use std::sync::Arc;
use hashbrown::HashMap;
use winit::event::{DeviceEvent, WindowEvent};
use winit::window::Window;
use crate::runtime::user_event::UserEvent;

pub trait AsyncApp<U: UserEvent = ()> {
    fn create(window: Arc<Window>) -> impl Future<Output = anyhow::Result<Self>>;

    fn on_window_event(&mut self, event: WindowEvent) -> impl Future<Output = anyhow::Result<()>>;

    fn on_device_event(&mut self, event: DeviceEvent) -> impl Future<Output = anyhow::Result<()>>;

    fn on_user_event(&mut self, _event: U::Output) -> impl Future<Output = anyhow::Result<()>> {}
}
