use std::net::SocketAddr;

use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;

    info!("Dredis: listening on: {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Acceptted connection from: {}", raddr);

        // tokio add task
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("Error processing conn with {}: {:?}", raddr, e);
            }
        });
    }
}

async fn process_redis_conn(mut stream: tokio::net::TcpStream, raddr: SocketAddr) -> Result<()> {
    // TODO
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf[..n]);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
    warn!("Connection closed from {}", raddr);
    Ok(())
}
