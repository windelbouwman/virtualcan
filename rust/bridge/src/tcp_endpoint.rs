use crate::can_frame::CanFrame;
use crate::client::{CanDevice, CanSink, CanSource};
use scroll::{Pread, Pwrite};
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct VirtualCanBus {
    socket: TcpStream,
}

#[derive(Debug)]
pub enum VirtualCanError {
    Io(std::io::Error),
    Other(String),
}

impl From<std::io::Error> for VirtualCanError {
    fn from(err: std::io::Error) -> Self {
        VirtualCanError::Io(err)
    }
}

impl From<scroll::Error> for VirtualCanError {
    fn from(err: scroll::Error) -> Self {
        VirtualCanError::Other(format!("Error: {}", err))
    }
}

impl VirtualCanBus {
    pub fn new(host: &str, port: u16) -> Result<Self, VirtualCanError> {
        use std::str::FromStr;
        let ip = std::net::IpAddr::from_str(host).unwrap();
        let addr = std::net::SocketAddr::new(ip, port);
        let stream = TcpStream::connect(addr)?;

        Ok(VirtualCanBus { socket: stream })
    }

    /// Length prefix emit a byte blob
    fn send_blob(&mut self, buf: &[u8]) -> Result<(), VirtualCanError> {
        let mut header: [u8; 4] = [0; 4];
        header.pwrite_with::<u32>(buf.len() as u32, 0, scroll::BE)?;
        self.socket.write_all(&header)?;
        self.socket.write_all(buf)?;
        Ok(())
    }

    /// Length prefixed receptical of a byte blob
    fn recv_blob(&mut self) -> Result<Vec<u8>, VirtualCanError> {
        let mut header: [u8; 4] = [0; 4];
        self.socket.read_exact(&mut header)?;
        let packet_size: u32 = header.pread_with::<u32>(0, scroll::BE)?;
        let mut buf = vec![0u8; packet_size as usize];
        self.socket.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl CanDevice for VirtualCanBus {
    fn dup(&self) -> Self {
        let sock = self.socket.try_clone().unwrap();
        VirtualCanBus { socket: sock }
    }

    fn close(&self) {
        self.socket.shutdown(std::net::Shutdown::Both);
    }
}

impl CanSink for VirtualCanBus {
    type Error = VirtualCanError;

    fn send(&mut self, frame: CanFrame) -> Result<(), VirtualCanError> {
        let buf = frame.to_bytes();
        self.send_blob(&buf)?;
        Ok(())
    }
}

impl CanSource for VirtualCanBus {
    type Error = VirtualCanError;

    fn recv(&mut self) -> Result<CanFrame, VirtualCanError> {
        let buf = self.recv_blob()?;
        let frame = CanFrame::from_bytes(&buf);
        Ok(frame)
    }
}
