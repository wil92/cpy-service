mod communication;
mod protocol;

use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{spawn, time};
use crate::communication::{connect_to_slave_service, start_slave_service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = true;

    if server {
        let listener = TcpListener::bind("127.0.0.1:4646").await.unwrap();
        loop {
            let (mut socket, _) = listener.accept().await?;

            spawn(async move {
                let mut buf = vec![0; 1024];

                loop {
                    let n = socket.read(&mut buf).await.unwrap();

                    if n == 0 {
                        return;
                    }

                    socket.write_all(&buf[0..n]).await.unwrap();
                }
            });
        }
    } else {
        // Start both master and slave services on separate tasks.
        let slave_task = spawn(start_slave_service("0.0.0.0:5000".parse().unwrap()));

        // Wait for a while to make sure the slave service is up and running.
        //time::delay_for(std::time::Duration::from_secs(1)).await;

        // Connect to the slave service and send a message.
        if let Err(e) = connect_to_slave_service("127.0.0.1:5000".parse().unwrap(), "Hello, Slave!").await {
            println!("Error connecting to the slave: {:?}", e);
        }

        // Wait for the slave task to finish before exiting the program.
        if let Err(e) = slave_task.await {
            println!("Error running slave service: {:?}", e);
        }
    }
    Ok(())
}
