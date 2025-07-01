use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct Channel<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn sender(&self) -> Sender<T> {
        self.tx.clone()
    }

    pub fn push(&self, value: T) -> Result<(), crossbeam::channel::TrySendError<T>> {
        self.tx.try_send(value)
    }
}

pub fn unbounded<T>() -> Channel<T> {
    let (tx, rx) = crossbeam::channel::unbounded();
    Channel { tx, rx }
}

pub fn bounded<T>(cap: usize) -> Channel<T> {
    let (tx, rx) = crossbeam::channel::bounded(cap);
    Channel { tx, rx }
}

impl<'a, T> IntoIterator for &'a Channel<T> {
    type Item = T;
    type IntoIter = crossbeam::channel::TryIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.rx.try_iter()
    }
}
