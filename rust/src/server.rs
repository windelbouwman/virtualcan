use bytes::Bytes;
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

pub fn run_server(port: u16) {
    let mut runtime = tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .thread_name("Tokio-server-thread")
        .build()
        .unwrap();

    runtime.block_on(async {
        if let Err(err) = server_prog(port).await {
            error!("Server stopped with error: {}", err);
        }
    });
}

struct Server {
    peers: Vec<Peer>,
}

impl Server {
    fn new() -> Server {
        Server { peers: vec![] }
    }

    fn add_peer(&mut self, peer: Peer) {
        self.peers.push(peer);
    }

    fn broadcast(&mut self, msg: Bytes) {
        for peer in &mut self.peers {
            peer.send_out(msg.clone());
        }
    }
}

struct Peer {
    tx: mpsc::UnboundedSender<Bytes>,
}

impl Peer {
    fn send_out(&mut self, msg: Bytes) {
        trace!("Peer sendout msg {:?}", msg);
        self.tx.unbounded_send(msg);
    }
}

enum ServerEvent {
    Message(Bytes),
    Peer(Peer),
}

async fn server_prog(port: u16) -> std::io::Result<()> {
    // let ip = std::net::Ipv6Addr::UNSPECIFIED;
    // let addr = std::net::SocketAddrV6::new(ip, port, 0, 0);
    let ip = std::net::Ipv4Addr::UNSPECIFIED;
    let addr = std::net::SocketAddrV4::new(ip, port);
    info!("Starting virtual can server at: {:?}", addr);
    let std_listener = std::net::TcpListener::bind(addr)?;
    let mut listener = TcpListener::from_std(std_listener)?;
    info!("Bound to {:?}", addr);

    let (broadcast_tx, distributor_rx) = mpsc::unbounded::<ServerEvent>();

    let _distributor_task_handle = tokio::spawn(async {
        let result = distributor_prog(distributor_rx).await;
        if let Err(err) = result {
            error!("Error in distribution task: {:?}", err);
        }
    });

    loop {
        let (client_socket, remote_addr) = listener.accept().await?;
        info!("New socket from: {:?}", remote_addr);
        process_socket(broadcast_tx.clone(), client_socket);
    }
}

async fn distributor_prog(mut rx: mpsc::UnboundedReceiver<ServerEvent>) -> std::io::Result<()> {
    let mut server = Server::new();
    while let Some(item) = rx.next().await {
        match item {
            ServerEvent::Message(msg) => {
                server.broadcast(msg);
            }
            ServerEvent::Peer(peer) => {
                server.add_peer(peer);
            }
        }
    }

    Ok(())
}

fn process_socket(broadcast_tx: mpsc::UnboundedSender<ServerEvent>, stream: TcpStream) {
    let _peer_task_handle = tokio::spawn(async {
        let result = peer_prog(stream, broadcast_tx).await;
        if let Err(err) = result {
            error!("Error in peer task: {:?}", err);
        }
    });
}

async fn peer_prog(
    mut stream: TcpStream,
    broadcast_tx: mpsc::UnboundedSender<ServerEvent>,
) -> std::io::Result<()> {
    // Create outbound channel:
    let (emit_tx, mut emit_rx) = mpsc::unbounded::<Bytes>();

    let peer = Peer { tx: emit_tx };
    broadcast_tx
        .unbounded_send(ServerEvent::Peer(peer))
        .unwrap();

    let (tcp_read, tcp_write) = stream.split();

    // Create packetizers:
    let mut packet_stream = FramedRead::new(tcp_read, LengthDelimitedCodec::new()).fuse();
    let mut packet_sink = FramedWrite::new(tcp_write, LengthDelimitedCodec::new());

    loop {
        futures::select! {
            optional_packet = packet_stream.next() => {
                if let Some(packet) = optional_packet {
                    let item = packet?.freeze();
                    trace!("Item! {:?}", item);
                    broadcast_tx
                    .unbounded_send(ServerEvent::Message(item))
                    .unwrap();
                } else {
                    info!("No more incoming packets.");
                    break;
                }
            }
            optional_item = emit_rx.next() => {
                if let Some(item) = optional_item {
                    trace!("BROADCAST: {:?}", item);
                    packet_sink.send(item).await;
                } else {
                    info!("No messages to broadcast.");
                    break;
                }
            }
        };
    }

    Ok(())
}
