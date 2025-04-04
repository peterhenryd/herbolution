use crate::client::input::ClientInputReceiver;
use crate::client::output::ClientOutputSender;
use crate::world::entity::set::EntityId;

pub mod input;
pub mod output;

pub struct Client {
    pub entity_id: EntityId,
    pub input_receiver: ClientInputReceiver,
    pub output_sender: ClientOutputSender,
}