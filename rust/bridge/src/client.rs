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
pub fn bridge(host: &str, port: u16, peer_host: &str, peer_port: u16) {
    let can0 = VirtualCanBus::new(host, port).unwrap();
    let can1 = VirtualCanBus::new(peer_host, peer_port).unwrap();

    let (can0_copy, can1_copy) = (can0.dup(), can1.dup());
    let t1 = std::thread::spawn(move || {
        if let Err(err) = chain_func(can0_copy, can1_copy) {
            error!("chain function ended with error: {}", err);
        }
        info!("Chain function completed!");
    });
    if let Err(err) = chain_func(can1, can0) {
        error!("chain function ended with error: {}", err);
    }
    info!("Chain function completed!");
    t1.join().unwrap();
}

#[cfg(target_os = "linux")]
pub fn bridge_can0(host: &str, port: u16, can_device: &str) {
    use crate::socket_can_endpoint::SocketCanBus;
    let can0 = VirtualCanBus::new(host, port).unwrap();
    let can1 = SocketCanBus::new(can_device);

    let (can0_copy, can1_copy) = (can0.dup(), can1.dup());
    let t1 = std::thread::spawn(move || {
        if let Err(err) = chain_func(can0_copy, can1_copy) {
            error!("chain function ended with error: {}", err);
        }
        info!("Chain function completed!");
    });
    if let Err(err) = chain_func(can1, can0) {
        error!("chain function ended with error: {}", err);
    }
    info!("Chain function completed!");
    t1.join().unwrap();
}

#[cfg(not(target_os = "linux"))]
pub fn bridge_can0(_host: &str, _port: u16, _can_device: &str) {
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

fn chain_func<I1, I2>(mut can0: I1, mut can1: I2) -> Result<(), String>
where
    I1: CanSource,
    I2: CanSink,
{
    loop {
        let frame = can0.recv().map_err(|e| format!("{:?}", e))?;
        can1.send(frame).map_err(|e| format!("{:?}", e))?;
    }
}
