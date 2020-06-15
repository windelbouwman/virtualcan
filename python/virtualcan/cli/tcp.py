""" Command line utils
"""

import argparse
import logging
import time
import asyncio
from ..can_message import CanMessage

from ..tcp import TcpClient, TcpServer


def tcp_can_server():
    """ Setup a virtual can server over TCP/IP """
    server = TcpServer()
    asyncio.run(server.run())


def tcp_can_client():
    async def task():
        client = TcpClient()
        await client.connect()
        count = 25000
        t1 = time.time()
        for x in range(count):
            can_message = CanMessage(
                1337 + x, False, bytes([1, 0xF2, 3, 0xCA, 0xFE, x % 255])
            )
            await client.send_message(can_message)
        t2 = time.time()
        time_delta = t2 - t1
        msg_per_second = count / time_delta
        print(f"Transmitted {msg_per_second} messages / second")
        # msg = await client.recv_message()
        # print('Message!', msg)
        await client.disconnect()

    asyncio.run(task())


def tcp_client_dump():
    logger = logging.getLogger("can dump over tcp")
    logger.info("Dumping can messages on TCP/IP virtual CAN bus")

    async def task():
        client = TcpClient()
        await client.connect()

        while True:
            msg = await client.recv_message()
            logger.info("Got message %s", msg)

        await client.disconnect()

    asyncio.run(task())
