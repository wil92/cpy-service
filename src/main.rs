use std::io;
use tokio::net::TcpListener;

mod communication;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4646").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket.read(&mut buf).await;

                if n == 0 {
                    return;
                }

                socket.write_all(&buf[0..n]).await;
            }
        });
    }
}
