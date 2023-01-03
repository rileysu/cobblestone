mod connector;
mod boundary;
mod codec;

use std::{thread::sleep, time::Duration};

use connector::connection_handler::ConnectionHandler;

use crate::boundary::message::OutboundMessage;

fn main() {
    let (mut main_boundary, connection_handler) = ConnectionHandler::bootstrap();

    loop {
        while let Some(message) = main_boundary.recieve_message() {

            println!("{message:?}");

            match message.message {
                boundary::message::InboundMessage::InitConnection { outbound_tx } => {
                    main_boundary.register_sender(&message.id, outbound_tx);
                }
                boundary::message::InboundMessage::ServerInformationRequest => {
                    main_boundary.send_message(&message.id, OutboundMessage::ServerInformationResponse {
                        version_name: "1.19.3".to_string(), 
                        version_protocol: 761,
                        players_max: 100, 
                        players_online: 0, 
                        sample: vec![], 
                        description_text: "Cobblestone Server".to_string(), 
                        favicon: "data:image/png;base64,<data>".to_string(), 
                        previews_chat: true, 
                        enforce_secure_chat: true, 
                    });
                },
                boundary::message::InboundMessage::PingRequest { payload } => {
                    main_boundary.send_message(&message.id, OutboundMessage::PingResponse { 
                        payload 
                    });
                },
                boundary::message::InboundMessage::LoginStart { 
                    username, 
                    uuid 
                } => {
                    main_boundary.send_message(&message.id, OutboundMessage::LoginSuccess {
                        username, 
                        uuid, 
                        properties: vec![],
                    });
                },
                boundary::message::InboundMessage::TermConnection => {
                    main_boundary.remove_sender(&message.id);
                },
            }
        }

        sleep(Duration::from_micros(625)) // 32 per tick!
    }
}

