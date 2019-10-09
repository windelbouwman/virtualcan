
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut stream = TcpStream::connect("127.0.0.1:8888")?;

    Ok(())
}
