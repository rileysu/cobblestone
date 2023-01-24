use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{Error, ErrorKind};
use tokio::runtime::Runtime;
use crate::boundary::BoundaryFactory;
use crate::boundary::connection_boundary::{RecieverConnectionBoundary, SenderConnectionBoundary};
use crate::boundary::main_boundary::MainBoundary;
use crate::boundary::message::InboundMessage;
use super::state_processor::{SenderProcessor, RecieverProcessor};

#[derive(Debug)]
pub struct ConnectionHandler {
    pub runtime: Arc<Runtime>,
}

const DATA_BITS: u8 = 0x7F;
const CONTINUE_BITS: u8 = 0x80;

async fn read_varint(reader: &mut OwnedReadHalf) -> Result<i32, Error> {
        let mut out = 0u32; //Unsigned for logical bit operations
        
        for pos in (0..32).step_by(7) {
            let val = reader.read_u8().await?;

            out = out | (((val & DATA_BITS) as u32) << pos);

            if val & CONTINUE_BITS == 0 {
                break;
            }
        }

        Ok(out as i32)
}

async fn write_varint(writer: &mut OwnedWriteHalf, value: i32) -> Result<(), Error> {
    let mut val = value as u32;
    
    for _ in (0..32).step_by(7) {
        if (val & !(DATA_BITS as u32)) == 0 {
            writer.write_u8(val as u8).await?;
            return Ok(())
        }

        writer.write_u8(((val as u8) & DATA_BITS) | CONTINUE_BITS).await?;

        val = val >> 7;
    }

    Err(ErrorKind::InvalidData.into())
}



// async fn is_connection_terminated(reader: &mut BufReader<OwnedReadHalf>) -> bool {
//     match reader.peek(&mut [0u8]).await {
//         Ok(0) => true,
//         _ => false,
//     }
// }

async fn handle_new_connections(runtime: Arc<Runtime>, boundary_factory: BoundaryFactory) {
    let listener = TcpListener::bind("0.0.0.0:25565").await.unwrap();

    loop {
        let (new_stream, addr) = listener.accept().await.unwrap();

        new_stream.set_nodelay(true).unwrap();

        let (reader, writer) = new_stream.into_split();
        let (sender_boundary, reciever_boundary) = boundary_factory.construct_connection_boundary(&addr.to_string());

        runtime.spawn(handle_reciever(reader, reciever_boundary));
        runtime.spawn(handle_sender(writer, sender_boundary));
    }
}

async fn handle_reciever(mut reader: OwnedReadHalf, boundary: RecieverConnectionBoundary) {
    let mut processor = RecieverProcessor::new();

    //let mut reader = BufReader::new(reader);

    loop {
        let length = match read_varint(&mut reader).await {
            Ok(length) => length,
            Err(_) => {
                boundary.send_message(InboundMessage::TermConnection).unwrap();
                break;
            }
        };

        let mut buf = vec![0u8; length as usize];
        match reader.read_exact(&mut buf).await {
            Err(_) => {
                boundary.send_message(InboundMessage::TermConnection).unwrap();
                break;
            },
            _ => {},
        };

        let optional_message = processor.process(&buf);

        if let Some(message) = optional_message {
            boundary.send_message(message).unwrap();
        }
    }
}

async fn handle_sender(mut writer: OwnedWriteHalf, mut boundary: SenderConnectionBoundary) {
    let mut processor = SenderProcessor::new();

    loop {
        match boundary.recieve_message().await {
            Some(message) => {
                let data = processor.process(&message);
                
                write_varint(&mut writer, data.len() as i32).await.unwrap();
                writer.write(&data).await.unwrap();
            },
            None => break,
        }
    }
}

impl ConnectionHandler {
    pub fn bootstrap() -> (MainBoundary, ConnectionHandler) {
        let (main_boundary, boundary_factory) = BoundaryFactory::new_main_boundary_and_factory();

        let connection_handler = ConnectionHandler {
            runtime: Arc::new(Runtime::new().unwrap()),
        };

        connection_handler.runtime.spawn(handle_new_connections(connection_handler.runtime.clone(), boundary_factory));

        (main_boundary, connection_handler)
    }
}