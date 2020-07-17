use crate::can_frame::CanFrame;
use crate::client::{CanDevice, CanSink, CanSource};
use socketcan;
use std::sync::Arc;

pub struct SocketCanBus {
    sock: Arc<socketcan::CANSocket>,
}

#[derive(Debug)]
pub enum SocketCanError {
    Io(std::io::Error),
    Other(String),
}

impl From<std::io::Error> for SocketCanError {
    fn from(err: std::io::Error) -> Self {
        SocketCanError::Io(err)
    }
}

impl SocketCanBus {
    /// connect to for example can0 or vcan0
    pub fn new(device: &str) -> Self {
        // Open socket can device
        let sock = Arc::new(socketcan::CANSocket::open(device).unwrap());

        SocketCanBus { sock }
    }
}

impl CanDevice for SocketCanBus {
    fn dup(&self) -> Self {
        let sock = self.sock.clone();
        SocketCanBus { sock }
    }

    fn close(&self) {
        // self.sock.close();
    }
}

impl CanSink for SocketCanBus {
    type Error = SocketCanError;

    fn send(&mut self, frame: CanFrame) -> Result<(), Self::Error> {
        let id: u32 = frame.id;

        // TODO:?
        // if frame.extended {
        //     id |= socketcan::EFF_FLAG;
        // }
        let data = &frame.data;
        let frame2 = socketcan::CANFrame::new(id, data, false, false).unwrap();

        // retry loop:
        loop {
            match self.sock.write_frame_insist(&frame2) {
                Ok(_) => break,
                Err(err) => {
                    let wait_time = std::time::Duration::from_millis(5);
                    // error, sleep, and retry later!
                    debug!(
                        "Error in sending can frame: {:?} retrying after {:?}!",
                        err, wait_time
                    );
                    std::thread::sleep(wait_time);
                }
            }
        }
        Ok(())
    }
}

impl CanSource for SocketCanBus {
    type Error = SocketCanError;

    fn recv(&mut self) -> Result<CanFrame, Self::Error> {
        let frame2 = self.sock.read_frame()?;

        let id: u32 = frame2.id();
        let extended: bool = frame2.is_extended();

        let data = frame2.data().to_vec();

        let frame = CanFrame { id, extended, data };

        Ok(frame)
    }
}
