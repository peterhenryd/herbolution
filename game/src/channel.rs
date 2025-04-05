use crossbeam::channel::{bounded, Receiver, Sender};
use tracing::warn;
use crate::client::input::ClientInputReceiver;
use crate::client::output::ClientOutputReceiver;

pub struct ClientChannel {
    client_input: Sender<ClientInputReceiver>,
    client_output: Receiver<ClientOutputReceiver>,
    exit_signal: Option<oneshot::Sender<()>>,
}

pub struct ServerChannel {
    client_input: Receiver<ClientInputReceiver>,
    client_output: Sender<ClientOutputReceiver>,
    exit_signal: oneshot::Receiver<()>,
}

impl ClientChannel {
    pub fn send_input_receiver(&self, receiver: ClientInputReceiver) {
        if let Err(e) = self.client_input.try_send(receiver) {
            warn!("Failed to send input receiver to client: {e}");
        }
    }

    pub fn recv_output_receiver(&self) -> Option<ClientOutputReceiver> {
        self.client_output.try_recv().ok()
    }

    pub fn send_exit_signal(&mut self) {
        let Some(sender) = self.exit_signal.take() else { return };

        if let Err(_) = sender.send(()) {
            warn!("Failed to send exit signal");
        }
    }
}

impl ServerChannel {
    pub fn send_output_receiver(&self, receiver: ClientOutputReceiver) {
        if let Err(e) = self.client_output.try_send(receiver) {
            warn!("Failed to send output receiver to server: {e}");
        }
    }

    pub fn recv_input_receiver(&self) -> Option<ClientInputReceiver> {
        self.client_input.try_recv().ok()
    }

    pub fn exit_requested(&mut self) -> bool {
        self.exit_signal.try_recv().is_ok()
    }
}

pub fn create() -> (ClientChannel, ServerChannel) {
    let (client_input_tx, client_input_rx) = bounded(1);
    let (client_output_tx, client_output_rx) = bounded(1);
    let (exit_signal_tx, exit_signal_rx) = oneshot::channel();

    (
        ClientChannel {
            client_input: client_input_tx,
            client_output: client_output_rx,
            exit_signal: Some(exit_signal_tx),
        },
        ServerChannel {
            client_input: client_input_rx,
            client_output: client_output_tx,
            exit_signal: exit_signal_rx,
        }
    )
}