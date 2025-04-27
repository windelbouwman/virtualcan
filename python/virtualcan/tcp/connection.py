"""A connection which can be shared by client and server."""

import logging
import asyncio
import struct


class Connection:
    """Wrapper to send chunks prefixed with a length."""

    _FMT = ">I"
    logger = logging.getLogger("tcp_chunked_connection")

    def __init__(self, reader: asyncio.StreamReader, writer: asyncio.StreamWriter):
        self._reader = reader
        self._writer = writer

    async def close(self):
        """Close this connection."""
        self._writer.close()
        await self._writer.wait_closed()

    async def send_message(self, data):
        data_len = len(data)
        header_data = struct.pack(self._FMT, data_len)
        # self.logger.debug(f"Sending {data_len} bytes")
        self._writer.write(header_data)
        self._writer.write(data)
        await self._writer.drain()

    async def recv_packet(self):
        header_data = await self._reader.readexactly(struct.calcsize(self._FMT))
        (data_len,) = struct.unpack(self._FMT, header_data)
        # self.logger.debug(f"Receiving {data_len} bytes")
        data = await self._reader.readexactly(data_len)
        return data
