use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod communication;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = true;

    if server {
        let listener = TcpListener::bind("127.0.0.1:4646").await.unwrap();
        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
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
        // todo
    }
    Ok(())
}
