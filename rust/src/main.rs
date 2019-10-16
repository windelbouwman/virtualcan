// use std::net::TcpStream;

use log::{info, trace};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::codec::{Framed, LengthDelimitedCodec};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::mpsc;
use bytes::Bytes;

struct Server {
    peers: Vec<Peer>,
    msg_counter: Mutex<usize>,
}

impl Server {
    fn new() -> Server {
        Server { peers: vec![], msg_counter: Mutex::new(0) }
    }

    fn broadcast(&self, msg: Bytes) {
        let mut l = self.msg_counter.lock().unwrap();
        *l += 1;
        trace!("Broadcast msg {}", *l);

        for peer in &self.peers {
            trace!("Broadcast to {}", *l);
            peer.send_out(msg.clone());
            // TODO!
        }
    }
}

struct Peer {
    stream: Framed<TcpStream, LengthDelimitedCodec>,
}

impl Peer {
    fn send_out(&self, msg: Bytes) {
        trace!("Peer sendout msg {:?}", msg);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init().unwrap();
    println!("Hello, world!");

    // let mut stream = TcpStream::connect("127.0.0.1:8888")?;

    let address = "127.0.0.1:8888";
    info!("Listening on {}", address);
    let addr = address.parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr)?;

    let server = Arc::new(Server::new());

    // accept connections and process them
    let server_loop = listener
        .incoming()
        .map_err(|e| eprintln!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let server = server.clone();
            process_socket(server, socket);
            Ok(())
        });

    tokio::run(server_loop);

    Ok(())
}

fn process_socket(server: Arc<Server>, stream: TcpStream) {
    // Create packetizer:
    let t = Framed::new(stream, LengthDelimitedCodec::new());
    // t.spl

    // Create outbound channel:
    let (tx, rx) = mpsc::unbounded_channel::<Bytes>();

    let tx_task = rx.for_each(|item| {
        // t.send
        // let tx_task = t.send_all(rx)
        Ok(())
    })
    .map_err(|e| {
        println!("Error in peer {}", e);
    });
    tokio::spawn(tx_task);

    // rx half of the connection:
    let rx_task = t.for_each(move |item| {
        trace!("Item! {:?}", item);
        server.broadcast(item.freeze());
        Ok(())
    })
    .map_err(|e| {
        println!("Error in peer {}", e);
    });

    tokio::spawn(rx_task);

    // let peer = Peer { stream: t };
}
