use crate::protocol;

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_slave_service(bind_address: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(bind_address).await?;
    println!("Slave service is listening on {}", bind_address);

    while let Ok((mut stream, _)) = listener.accept().await {
        let mut data = [0u8; 200];
        let size = stream.read(&mut data).await?;

        if size > 0 {
            let (decoded_messages, _) = protocol::decode_string(&data[..size], size);
            for (msg, id, flags, addr, port) in decoded_messages {
                println!("Slave Received: {:?}", msg);
            }
        }
    }

    Ok(())
}

pub async fn connect_to_slave_service(target_address: SocketAddr, msg_to_send: &str) -> Result<(), Box<dyn std::error::Error>> {
    match TcpStream::connect(target_address).await {
        Ok(mut stream) => {
            let data = msg_to_send.as_bytes();
            let id_connection: u16 = 1; // You can set the connection ID as needed.
            let flags: u8 = 0; // Set any flags if required.
            let addr: u32 = 0; // Set the destination address if required.
            let port: u16 = 0; // Set the destination port if required.

            let encoded_messages = protocol::code_string(data, data.len(), id_connection, flags, addr, port);

            for msg in encoded_messages {
                let mut buffer: [u8; 200] = [0; 200];
                let mut pos = 0;
                for byte in msg.clone().into_iter() {
                    buffer[pos] = byte;
                    pos += 1;
                }
                stream.write_all(&buffer).await?;

                // Message sent successfully
                println!("Master Sent: {:?}", msg);
            }
            Ok(())
        },
        Err(e) => {
            println!("Error connecting to the slave: {:?}", e);
            Err(Box::new(e))
        }
    }
}