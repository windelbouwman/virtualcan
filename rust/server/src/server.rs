use bytes::Bytes;
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

pub fn run_server(port: u16) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        if let Err(err) = server_prog(port).await {
            error!("Server stopped with error: {}", err);
        }
    });
}

struct Server {
    peers: HashMap<u32, Peer>,
}

impl Server {
    fn new() -> Server {
        Server {
            peers: HashMap::new(),
        }
    }

    fn handle_event(&mut self, event: ServerEvent) {
        match event {
            ServerEvent::Message { source_port, msg } => {
                self.broadcast(source_port, msg);
            }
            ServerEvent::AddPeer(peer) => {
                self.peers.insert(peer.port_id, peer);
            }
            ServerEvent::RemovePeer(id) => {
                self.peers.remove(&id);
            }
        }
    }

    fn broadcast(&mut self, source_port: u32, msg: Bytes) {
        for peer in &mut self.peers.values_mut() {
            if let Err(_err) = peer.send_out(source_port, msg.clone()) {
                info!("Peer disconnect");
            }
        }
    }
}

struct Peer {
    tx: mpsc::UnboundedSender<Bytes>,

    /// A unique ID for this peers 'port'
    port_id: u32,
}

impl Peer {
    fn send_out(&mut self, source_port: u32, msg: Bytes) -> Result<(), ()> {
        trace!("Peer sendout msg {:?}", msg);
        if source_port != self.port_id {
            if let Err(_err) = self.tx.unbounded_send(msg) {
                return Err(());
            }
        }

        Ok(())
    }
}

enum ServerEvent {
    Message { source_port: u32, msg: Bytes },
    AddPeer(Peer),
    RemovePeer(u32),
}

async fn server_prog(port: u16) -> std::io::Result<()> {
    // let ip = std::net::Ipv6Addr::UNSPECIFIED;
    // let addr = std::net::SocketAddrV6::new(ip, port, 0, 0);
    let ip = std::net::Ipv4Addr::UNSPECIFIED;
    let addr = std::net::SocketAddrV4::new(ip, port);
    info!("Starting virtual can server at: {:?}", addr);
    let std_listener = std::net::TcpListener::bind(addr)?;
    std_listener.set_nonblocking(true)?;
    let listener = tokio::net::TcpListener::from_std(std_listener)?;
    info!("Bound to {:?}", addr);

    let (broadcast_tx, distributor_rx) = mpsc::unbounded::<ServerEvent>();

    let _distributor_task_handle = tokio::spawn(async {
        distributor_prog(distributor_rx).await;
    });

    let mut peer_counter: u32 = 0;
    loop {
        let (client_socket, remote_addr) = listener.accept().await?;
        info!(
            "New socket from: {:?} --> id = {}",
            remote_addr, peer_counter
        );
        process_socket(broadcast_tx.clone(), client_socket, peer_counter);
        peer_counter += 1;
    }
}

async fn distributor_prog(mut rx: mpsc::UnboundedReceiver<ServerEvent>) {
    let mut server = Server::new();
    while let Some(event) = rx.next().await {
        server.handle_event(event);
    }
}

fn process_socket(
    server_cmds: mpsc::UnboundedSender<ServerEvent>,
    stream: tokio::net::TcpStream,
    peer_id: u32,
) {
    let _peer_task_handle = tokio::spawn(async move {
        let result = peer_prog(stream, server_cmds.clone(), peer_id).await;
        if let Err(err) = result {
            error!("Error in peer task: {:?}", err);
        }
        server_cmds
            .unbounded_send(ServerEvent::RemovePeer(peer_id))
            .unwrap();
    });
}

async fn peer_prog(
    mut stream: tokio::net::TcpStream,
    server_cmds: mpsc::UnboundedSender<ServerEvent>,
    peer_id: u32,
) -> std::io::Result<()> {
    // Create outbound channel:
    let (emit_tx, mut emit_rx) = mpsc::unbounded::<Bytes>();

    let peer = Peer {
        tx: emit_tx,
        port_id: peer_id,
    };
    server_cmds.unbounded_send(ServerEvent::AddPeer(peer)).unwrap();

    stream.set_nodelay(true)?;
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
                    server_cmds
                    .unbounded_send(ServerEvent::Message { source_port: peer_id, msg: item } )
                    .unwrap();
                } else {
                    info!("No more incoming packets.");
                    break;
                }
            }
            optional_item = emit_rx.next() => {
                if let Some(item) = optional_item {
                    trace!("BROADCAST: {:?}", item);
                    packet_sink.send(item).await.expect("Send should work");
                } else {
                    info!("No messages to broadcast.");
                    break;
                }
            }
        };
    }

    Ok(())
}
