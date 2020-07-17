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

pub trait CanDevice {
    fn dup(&self) -> Self;
    fn close(&self);
}

// trait CanInterface  = CanSink + CanSource;

/// Bridge socket can to tcp / ip can port!
pub fn bridge(host: &str, port: u16, peer_host: &str, peer_port: u16) {
    let can0 = VirtualCanBus::new(host, port).unwrap();
    let can1 = VirtualCanBus::new(peer_host, peer_port).unwrap();

    run_bridging(can0, can1);
}

pub fn run_bridging<C0, C1>(can0: C0, can1: C1)
where
    C0: CanDevice + CanSink + CanSource + Send + 'static,
    C1: CanDevice + CanSink + CanSource + Send + 'static,
{
    let (can0_copy, can1_copy) = (can0.dup(), can1.dup());

    let t1 = std::thread::spawn(move || {
        if let Err(err) = chain_func(can0_copy.dup(), can1_copy.dup()) {
            error!("chain function ended with error: {}", err);
        }
        info!("Chain function completed!");
        can0_copy.close();
        can1_copy.close();
    });

    let t2 = std::thread::spawn(move || {
        if let Err(err) = chain_func(can1.dup(), can0.dup()) {
            error!("chain function ended with error: {}", err);
        }
        info!("Chain function completed!");
        can0.close();
        can1.close();
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

#[cfg(target_os = "linux")]
pub mod socketcan {
    use super::{run_bridging, CanDevice, VirtualCanBus};

    pub fn bridge_can0(host: &str, port: u16, can_device: &str) {
        loop {
            if let Err(err) = try_to_bridge(host, port, can_device) {
                error!("Error in bridge: {:?}", err);
            } else {
                info!("Bridge completed ok");
            }
            let delay = std::time::Duration::from_secs(5);
            warn!("Bridge completed, retrying after {:?}", delay);
            std::thread::sleep(delay);
        }
    }

    fn try_to_bridge(host: &str, port: u16, can_device: &str) -> Result<(), String> {
        use crate::socket_can_endpoint::SocketCanBus;
        let can0 = VirtualCanBus::new(host, port).map_err(|e| format!("{:?}", e))?;
        let can1 = SocketCanBus::new(can_device);

        run_bridging(can0, can1);

        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
pub mod socketcan {
    pub fn bridge_can0(_host: &str, _port: u16, _can_device: &str) {
        unimplemented!("Not running on linux!");
    }
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
