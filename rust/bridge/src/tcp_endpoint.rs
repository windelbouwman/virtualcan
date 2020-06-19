use std::net::TcpStream;
// use futures::channel::mpsc;
// use futures::{SinkExt, StreamExt};
// use tokio::net::{TcpListener, TcpStream};
// use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::can_frame::CanFrame;
use crate::client::{CanSink, CanSource};
use scroll::{Pread, Pwrite};
use std::io::{Read, Write};

pub struct VirtualCanBus {
    socket: TcpStream,
    // tx_q: mpsc::Sender<CanFrame>,
    // rx_q: mpsc::Receiver<CanFrame>,
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

        // let (tcp_writer, tcp_reader) = stream.split();

        // // tx path:
        // let (tx_send, tx_recv) = mpsc::channel(11);

        // // rx path:
        // let (rx_send, rx_recv) = mpsc::channel(11);

        // let mut packet_sink = FramedWrite::new(tcp_writer, LengthDelimitedCodec::new());
        // tx_task(tx_recv, packet_sink);

        Ok(VirtualCanBus {
            socket: stream,
            // tx_q: tx_send,
            // rx_q: rx_recv,
        })
    }

    pub fn dup(&self) -> Self {
        let sock = self.socket.try_clone().unwrap();
        VirtualCanBus { socket: sock }
    }

    fn close(&self) {
        unimplemented!();
    }
}

impl CanSink for VirtualCanBus {
    type Error = VirtualCanError;

    fn send(&mut self, frame: CanFrame) -> Result<(), VirtualCanError> {
        // serialize frame:
        let buf = frame.to_bytes();

        // Length prefix emit:
        let mut header: [u8; 4] = [0; 4];
        header.pwrite_with::<u32>(buf.len() as u32, 0, scroll::BE)?;
        self.socket.write_all(&header)?;
        self.socket.write_all(&buf)?;
        Ok(())
    }
}

impl CanSource for VirtualCanBus {
    type Error = VirtualCanError;

    fn recv(&mut self) -> Result<CanFrame, VirtualCanError> {
        // self.rx_q.recv()

        // Length prefixed receptical:
        let mut header: [u8; 4] = [0; 4];
        self.socket.read_exact(&mut header)?;
        let packet_size: u32 = header.pread_with::<u32>(0, scroll::BE)?;
        // println!("Recv packet size: {}", packet_size);
        let mut buf = vec![0u8; packet_size as usize];
        self.socket.read_exact(&mut buf)?;

        // println!("Recv packet: {:?}", buf);

        // Deserialize frame:
        let frame = CanFrame::from_bytes(&buf);
        Ok(frame)
    }
}

// Send packets from queue to outside world!
// async fn tx_task(tx_recv: mpsc::Receiver<CanFrame>, tcp_sink: isize) {
//     loop {
//         let msg = tx_recv.next().await;
//         tcp_end.send(msg).await;
//     }
// }
