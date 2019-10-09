import asyncio
from .connection import Connection


class TcpServer:
    """ Central TCP server for CAN messages!
    """

    def __init__(self):
        self._peers = []

    async def run(self):
        port = 8888
        server = await asyncio.start_server(self.handle_peer, "127.0.0.1", port)
        async with server:
            await server.serve_forever()

    async def handle_peer(self, reader, writer):
        connection = Connection(reader, writer)
        peer = Peer(connection)
        self._peers.append(peer)
        try:
            while True:
                pkt = await connection.recv_packet()
                print("PKT!", pkt)
                for remote in self._peers:
                    await remote.send_message(pkt)
        finally:
            self._peers.remove(peer)


class Peer:
    """ A single connected client!
    """

    def __init__(self, connection):
        self._connection = connection

    async def send_message(self, packet):
        await self._connection.send_message(packet)
