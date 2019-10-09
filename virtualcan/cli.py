""" Command line utils
"""

import argparse
import logging
import asyncio
from .can_message import CanMessage

from .zmq import ZmqCanConnection, ZmqCanServer
from .tcp import TcpClient, TcpServer


def tcp_can_server():
    """ Setup a virtual can server over TCP/IP """
    server = TcpServer()
    asyncio.run(server.run())


def tcp_can_client():
    async def task():
        client = TcpClient()
        await client.connect()
        for x in range(1000):
            can_message = CanMessage(1337 + x, False, bytes([1, 0xF2, 3, 0xCA, 0xFE]))
            await client.send_message(can_message)

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


def zmq_can_server():
    """ Start a zeromq can-bus server.
    """
    server = ZmqCanServer()

    while True:
        # Wait for can message:
        server.logger.debug("awaiting message..")
        server.handle_rpc_call()

    server.context.term()


def zmq_client():
    client = ZmqCanConnection()
    client.connect()

    for x in range(10):
        can_msg = CanMessage(x, False, bytes([1, 2, 3, x]))
        client.send(can_msg)
    client.disconnect()


def zmq_client_dump():
    logger = logging.getLogger("zmq_can_dump")
    logger.info("Dumping can messages on zero mq can bus")
    client = ZmqCanConnection()
    client.connect()

    while True:
        msg = client.recv()
        logger.info("Got message %s", msg)

    client.disconnect()
