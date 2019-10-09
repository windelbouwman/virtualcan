import logging
import asyncio
from .connection import Connection
from ..can_message import CanMessage


class TcpClient:
    async def connect(self):
        hostname, port = "127.0.0.1", 8888
        reader, writer = await asyncio.open_connection(hostname, port)
        self._connection = Connection(reader, writer)

    async def disconnect(self):
        # TODO?
        pass

    async def send_message(self, can_message):
        bindata = can_message.to_bytes()
        await self._connection.send_message(bindata)

    async def recv_message(self):
        bindata = await self._connection.recv_packet()
        can_message = CanMessage.from_bytes(bindata)
        return can_message
