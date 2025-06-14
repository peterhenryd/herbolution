use crossbeam::channel::{bounded, Receiver, Sender};
use tracing::warn;
use crate::client::{ClientInputReceiver, ClientOutputReceiver};

#[derive(Debug)]
pub struct ClientChannel {
    client_input: Sender<ClientInputReceiver>,
    client_output: Receiver<ClientOutputReceiver>,
    exit: Option<oneshot::Sender<()>>,
}

pub struct ServerChannel {
    client_input: Receiver<ClientInputReceiver>,
    client_output: Sender<ClientOutputReceiver>,
    exit: oneshot::Receiver<()>,
}

impl ClientChannel {
    pub fn send_client_input(&self, input: ClientInputReceiver) {
        if let Err(e) = self.client_input.try_send(input) {
            warn!("Failed to send client input: {}", e);
        }
    }

    pub fn recv_client_output(&self) -> Option<ClientOutputReceiver> {
        self.client_output.try_recv().ok()
    }

    pub fn send_exit(&mut self) {
        let Some(sender) = self.exit.take() else { return };

        if let Err(e) = sender.send(()) {
            warn!("Failed to send exit signal: {}", e);
        }
    }
}

impl ServerChannel {
    pub fn recv_client_input(&self) -> Option<ClientInputReceiver> {
        self.client_input.try_recv().ok()
    }

    pub fn send_client_output(&self, output: ClientOutputReceiver) {
        if let Err(e) = self.client_output.try_send(output) {
            warn!("Failed to send client output: {}", e);
        }
    }

    pub fn recv_exit(&self) -> bool {
        self.exit.try_recv().is_ok()
    }
}

pub fn create() -> (ClientChannel, ServerChannel) {
    let (client_input_tx, client_input_rx) = bounded(1);
    let (client_output_tx, client_output_rx) = bounded(1);
    let (exit_tx, exit_rx) = oneshot::channel();

    (
        ClientChannel {
            client_input: client_input_tx,
            client_output: client_output_rx,
            exit: Some(exit_tx),
        },
        ServerChannel {
            client_input: client_input_rx,
            client_output: client_output_tx,
            exit: exit_rx,
        },
    )
}