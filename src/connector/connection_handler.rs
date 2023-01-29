use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncRead, AsyncWrite};
use tokio::io::{Error, ErrorKind};
use tokio::runtime::Runtime;
use crate::boundary::BoundaryFactory;
use crate::boundary::connection_boundary::{RecieverConnectionBoundary, SenderConnectionBoundary};
use crate::boundary::main_boundary::MainBoundary;
use crate::boundary::message::InboundMessage;
use crate::data::base::Uuid;
use super::new_connection_processor::{NewConnectionProcessor};
use super::sender_reciever_processing;

#[derive(Debug)]
pub struct ConnectionHandler {
    pub runtime: Arc<Runtime>,
}

const DATA_BITS: u8 = 0x7F;
const CONTINUE_BITS: u8 = 0x80;

async fn read_varint(reader: &mut (impl AsyncRead + Unpin)) -> Result<i32, Error> {
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

async fn write_varint(writer: &mut (impl AsyncWrite + Unpin), value: i32) -> Result<(), Error> {
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

async fn read_packet(mut reader: &mut (impl AsyncRead + Unpin)) -> Result<Vec<u8>, Error> {
    let length = read_varint(&mut reader).await?;
    let mut buf = vec![0u8; length as usize];

    reader.read_exact(&mut buf).await?;

    Ok(buf)
}

async fn write_packet(writer: &mut (impl AsyncWrite + Unpin), buf: &Vec<u8>) -> Result<(), Error> {
    write_varint(writer, buf.len() as i32).await?;
    writer.write(buf).await?;

    Ok(())
}

async fn handle_new_connections(runtime: Arc<Runtime>, boundary_factory: BoundaryFactory) {
    let listener = TcpListener::bind("0.0.0.0:25565").await.unwrap();

    loop {
        let (mut new_stream, _addr) = listener.accept().await.unwrap();

        new_stream.set_nodelay(true).unwrap();

        match handle_new_connection(&mut new_stream).await {
            Some(uuid) => {
                let (reader, writer) = new_stream.into_split();
                let (sender_boundary, reciever_boundary) = boundary_factory.construct_connection_boundaries(uuid);
        
                runtime.spawn(handle_reciever(reader, reciever_boundary));
                runtime.spawn(handle_sender(writer, sender_boundary));
            },
            None => continue,
        }
    }
}

async fn handle_new_connection(mut stream: &mut TcpStream) -> Option<Uuid> {
    let mut new_connection_processor = NewConnectionProcessor::new();

    loop {
        let inbound_packet_buf = match read_packet(&mut stream).await {
            Ok(buf) => buf,
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof | ErrorKind::ConnectionReset => return None,
                _ => panic!("{:?}", err),
            },
        };

        let (outbound_packet_bufs, maybe_login_info) =  new_connection_processor.process(&inbound_packet_buf);

        for packet_buf in outbound_packet_bufs {
            write_packet(&mut stream, &packet_buf).await.unwrap();
        }

        if let Some(login_info) = maybe_login_info {
            return Some(login_info);
        }
    }
}

async fn handle_reciever(mut reader: OwnedReadHalf, boundary: RecieverConnectionBoundary) {
    loop {
        let inbound_packet_buf = match read_packet(&mut reader).await {
            Ok(buf) => buf,
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => {
                    boundary.send_message(InboundMessage::TermConnection).unwrap();
                    return;
                },
                _ => panic!("{:?}", err),
            },
        };

        let message = match sender_reciever_processing::recieve_process(&inbound_packet_buf) {
            Ok(message) => message,
            Err(_) => {
                boundary.send_message(InboundMessage::TermConnection).unwrap();
                return;
            }
        };

        boundary.send_message(message).unwrap();
    }
}

async fn handle_sender(mut writer: OwnedWriteHalf, mut boundary: SenderConnectionBoundary) {
    loop {
        match boundary.recieve_message().await {
            Some(message) => {
                let action = sender_reciever_processing::send_process(&message).unwrap();

                match action {
                    sender_reciever_processing::NextAction::Send(outbound_packet_buf) => write_packet(&mut writer, &outbound_packet_buf).await.unwrap(),
                    sender_reciever_processing::NextAction::Kill => todo!(),
                }
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