use crate::can_frame::CanFrame;
use crate::tcp_endpoint::VirtualCanBus;

pub trait CanSource {
    type Error: std::fmt::Debug;
    fn recv(&mut self) -> Result<CanFrame, Self::Error>;
}

pub trait CanSink {
    type Error: std::fmt::Debug;
    fn send(&mut self, frame: CanFrame) -> Result<(), Self::Error>;
}

// trait CanInterface  = CanSink + CanSource;

/// Bridge socket can to tcp / ip can port!
pub fn bridge(host: &str, port: u16) {
    let can0 = VirtualCanBus::new(host, port).unwrap();
    let can1 = VirtualCanBus::new("127.0.0.1", 18882).unwrap();

    let (can0_copy, can1_copy) = (can0.dup(), can1.dup());
    let _t1 = std::thread::spawn(move || {
        chain_func(can0_copy, can1_copy);
    });
    chain_func(can1, can0);
    // _t1.join();
}

#[cfg(target_os = "linux")]
pub fn bridge_can0(host: &str, port: u16) {
    use crate::socket_can_endpoint::SocketCanBus;
    let can0 = VirtualCanBus::new(host, port).unwrap();
    let can1 = SocketCanBus::new("can0");

    let (can0_copy, can1_copy) = (can0.dup(), can1.dup());
    let _t1 = std::thread::spawn(move || {
        chain_func(can0_copy, can1_copy);
    });
    chain_func(can1, can0);
    // _t1.join();
}

#[cfg(not(target_os = "linux"))]
pub fn bridge_can0(_host: &str, _port: u16) {
    unimplemented!("Not running on linux!");
}

// fn run_bridge()

pub fn dump(host: &str, port: u16) {
    let can0 = VirtualCanBus::new(host, port).unwrap();
    can_dump(can0);
}

fn can_dump<S>(mut can0: S)
where
    S: CanSource,
{
    loop {
        let frame = can0.recv().unwrap();
        info!("FRAME: {:?}", frame);
    }
}

fn chain_func<I1, I2>(mut can0: I1, mut can1: I2)
where
    I1: CanSource,
    I2: CanSink,
{
    loop {
        let frame = can0.recv().unwrap();
        can1.send(frame).unwrap();
    }
}
