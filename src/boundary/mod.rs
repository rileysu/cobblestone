use crate::codec_data::base::Uuid;

use self::{main_boundary::MainBoundary, message::{OutboundMessage, InboundMessage}, connection_boundary::{RecieverConnectionBoundary, SenderConnectionBoundary}};
use tokio::sync::mpsc;

pub mod connection_boundary;
pub mod main_boundary;
pub mod message;



pub struct BoundaryFactory {
    inbound_tx: mpsc::UnboundedSender<(Uuid, InboundMessage)>,
    channel_tx: mpsc::UnboundedSender<(Uuid, mpsc::UnboundedSender<OutboundMessage>)>,
}

impl BoundaryFactory {
    pub fn new_main_boundary_and_factory() -> (MainBoundary, Self) {
        let (inbound_tx, inbound_rx) = mpsc::unbounded_channel::<(Uuid, InboundMessage)>();
        let (channel_tx, channel_rx) = mpsc::unbounded_channel::<(Uuid, mpsc::UnboundedSender<OutboundMessage>)>();

        (
            MainBoundary::new(inbound_rx, channel_rx),
            Self {
                inbound_tx,
                channel_tx,
            }
        )
    }

    pub fn construct_connection_boundaries(&self, uuid: Uuid) -> (SenderConnectionBoundary, RecieverConnectionBoundary) {
        let (outbound_tx, outbound_rx) = mpsc::unbounded_channel::<OutboundMessage>();

        self.channel_tx.send((uuid, outbound_tx)).unwrap();
        
        (
            SenderConnectionBoundary::new(outbound_rx),
            RecieverConnectionBoundary::new(uuid, self.inbound_tx.clone())
        )
    }
}