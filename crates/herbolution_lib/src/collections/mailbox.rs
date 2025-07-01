use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct Mailbox<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> Mailbox<T> {
    pub fn sender(&self) -> Sender<T> {
        self.tx.clone()
    }

    pub fn push(&self, value: T) -> Result<(), crossbeam::channel::TrySendError<T>> {
        self.tx.try_send(value)
    }
}

pub fn unbounded<T>() -> Mailbox<T> {
    let (tx, rx) = crossbeam::channel::unbounded();
    Mailbox { tx, rx }
}

pub fn bounded<T>(cap: usize) -> Mailbox<T> {
    let (tx, rx) = crossbeam::channel::bounded(cap);
    Mailbox { tx, rx }
}

impl<'a, T> IntoIterator for &'a Mailbox<T> {
    type Item = T;
    type IntoIter = crossbeam::channel::TryIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.rx.try_iter()
    }
}
