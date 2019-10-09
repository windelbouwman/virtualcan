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
        peer.start()
        self._peers.append(peer)
        try:
            while True:
                pkt = await connection.recv_packet()
                print("PKT!", pkt)
                for remote in self._peers:
                    if remote is not peer:
                        await remote.send_message(pkt)
        finally:
            self._peers.remove(peer)


class Peer:
    """ A single connected client!
    """

    def __init__(self, connection):
        self._connection = connection
        self._tx_queue = asyncio.Queue()
        self._tx_counter = 0

    def start(self):
        """ Start tx processing of this peer! """
        self._tx_task = asyncio.create_task(self._tx_task_func())

    async def send_message(self, packet):
        """ Schedule a packet for transmission! """
        await self._tx_queue.put(packet)

    async def _tx_task_func(self):
        while True:
            # Process transmit packet one at a time!
            packet = await self._tx_queue.get()
            await self._connection.send_message(packet)
            self._tx_queue.task_done()
            self._tx_counter += 1
            # print('Tx message:', self._tx_counter)
