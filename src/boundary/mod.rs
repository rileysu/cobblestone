use self::{main_boundary::MainBoundary, message::{IdentifiedInboundMessage, OutboundMessage, InboundMessage}, connection_boundary::{RecieverConnectionBoundary, SenderConnectionBoundary}};
use tokio::sync::mpsc;

pub mod connection_boundary;
pub mod main_boundary;
pub mod message;



pub struct BoundaryFactory {
    inbound_tx: mpsc::UnboundedSender<IdentifiedInboundMessage>,
}

impl BoundaryFactory {
    pub fn new_main_boundary_and_factory() -> (MainBoundary, Self) {
        let (inbound_tx, inbound_rx) = mpsc::unbounded_channel::<IdentifiedInboundMessage>();
        
        (
            MainBoundary::new(inbound_rx),
            Self {
                inbound_tx,
            }
        )
    }

    pub fn construct_connection_boundary(&self, id: &str) -> (SenderConnectionBoundary, RecieverConnectionBoundary) {
        let (outbound_tx, outbound_rx) = mpsc::unbounded_channel::<OutboundMessage>();

        self.inbound_tx.send( IdentifiedInboundMessage {
            id: id.to_string(),
            message: InboundMessage::InitConnection { 
                outbound_tx,
            },
        }).unwrap();
        
        (
            SenderConnectionBoundary::new(outbound_rx),
            RecieverConnectionBoundary::new(id, self.inbound_tx.clone())
        )
    }
}