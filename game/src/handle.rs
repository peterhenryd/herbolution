use crate::client::input::ClientInputReceiver;
use crate::client::output::ClientOutputReceiver;
use crate::world::chunk::ChunkUpdate;
use crossbeam::channel::{Receiver, Sender};
use math::vector::vec3i;

pub struct GameHandle {
    pub(super) request_tx: Sender<Request>,
    pub(super) response_rx: Receiver<Response>,
    pub(crate) shutdown_tx: Sender<()>,
}

impl GameHandle {
    pub fn exit(&self) {
        let _ = self.shutdown_tx.try_send(());
    }

    pub fn request_player(&self, receiver: ClientInputReceiver) {
        let _ = self.request_tx.try_send(Request::AddClient(receiver));
    }

    pub fn receive_response(&mut self) -> Option<Response> {
        self.response_rx.try_recv().ok()
    }
}

pub enum Request {
    AddClient(ClientInputReceiver),
}

pub enum Response {
    ClientAdded(ClientOutputReceiver),
    LoadChunk { position: vec3i, receiver: Receiver<ChunkUpdate> },
    UnloadChunk { position: vec3i },
}