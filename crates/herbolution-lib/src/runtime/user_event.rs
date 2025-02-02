use winit::event_loop::ActiveEventLoop;
use crate::runtime::Runtime;

pub trait UserEvent {
    type Output;

    fn process<A>(self, runtime: &mut Runtime<A, Self>, event_loop: &ActiveEventLoop) -> Self::Output;
}

impl UserEvent for () {
    type Output = ();

    fn process<A>(self, _: &mut Runtime<A, Self>, _: &ActiveEventLoop) -> Self::Output { () }
}