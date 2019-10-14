
import argparse
import logging
import time
import asyncio
from ..can_message import CanMessage
from ..zmq import ZmqCanConnection, ZmqCanServer


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
