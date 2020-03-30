import logging
import asyncio
from .connection import Connection
from ..can_message import CanMessage


class TcpClient:
    def __init__(self):
        self._connection = None
        self._running = False
        self._rx_queue = asyncio.Queue()

    async def connect(self):
        hostname, port = "127.0.0.1", 8888
        reader, writer = await asyncio.open_connection(hostname, port)
        self._connection = Connection(reader, writer)
        self._receiver_task = asyncio.create_task(self._recv_task_func())

    async def disconnect(self):
        # TODO?
        await self._connection.close()
        self._running = False
        await self._receiver_task

    async def send_message(self, can_message):
        """ Transmit a CAN message. """
        bindata = can_message.to_bytes()
        await self._connection.send_message(bindata)

    async def recv_message(self):
        """ Receive a CAN message. """
        can_message = await self._rx_queue.get()
        self._rx_queue.task_done()
        return can_message

    async def _recv_task_func(self):
        self._running = True
        while self._running:
            try:
                bindata = await self._connection.recv_packet()
                can_message = CanMessage.from_bytes(bindata)
                # print("Got message!", can_message)
                await self._rx_queue.put(can_message)
            except asyncio.IncompleteReadError:
                break
