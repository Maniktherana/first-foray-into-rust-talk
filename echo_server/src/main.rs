use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Bind the listener to the address
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    
    println!("Server listening on: {}", &listener.local_addr()?);

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                tokio::spawn(async move { echo(&mut socket, addr).await });
            }
            Err(e) => println!("Error accepting connection: {}", e),
        }
    }
}

async fn echo(socket: &mut TcpStream, address: SocketAddr) -> io::Result<()> {
    println!("New client: {}", address);

    let mut buffer = [0; 1024]; // Buffer to read/write data

    println!("Connected: {}", address);

    loop {
        let bytes_read = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => break, // End of stream
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return Err(e);
            }
        };

        match socket.write_all(&buffer[..bytes_read]).await {
            Ok(_) => {} // Data successfully written
            Err(e) => {
                eprintln!("Failed to write to socket: {}", e);
                return Err(e);
            }
        }
    }

    println!("Closed connection: {}", address);
    Ok(())
}
