use crossbeam_channel::{Receiver, Sender, TryIter, TrySendError, bounded, unbounded};

#[derive(Debug)]
pub struct Mailbox<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> Mailbox<T> {
    pub fn new(tx: Sender<T>, rx: Receiver<T>) -> Mailbox<T> {
        Mailbox { tx, rx }
    }

    pub fn with_capacity(capacity: usize) -> Mailbox<T> {
        let (tx, rx) = bounded(capacity);
        Mailbox { tx, rx }
    }
}

impl<T> Default for Mailbox<T> {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Mailbox { tx, rx }
    }
}

impl<T> Mailbox<T> {
    pub fn sender(&self) -> Sender<T> {
        self.tx.clone()
    }

    pub fn push(&self, value: T) -> Result<(), TrySendError<T>> {
        self.tx.try_send(value)
    }
}

impl<'a, T> IntoIterator for &'a Mailbox<T> {
    type Item = T;
    type IntoIter = TryIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.rx.try_iter()
    }
}
