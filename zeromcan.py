"""
Idea: Startup a zmq server, in pub sub mode, and publish can messages on this bus in messagepack format.
"""

import zmq
import msgpack
import coloredlogs
import logging
import argparse
import threading
import time
import can


class CanMessage:
    def __init__(self, id, extended, data):
        self.id = id
        self.extended = extended
        self.data = data
    
    def __repr__(self):
        return f'Can message id={self.id} extended={self.extended} data={self.data}'


API_GET_VERSION = 1
API_NEW_CLIENT = 2
API_SEND_CAN_MESSAGE = 3


class ZmqCanConnection:
    """ ZeroMQ can bus client.
    """
    logger = logging.getLogger('zmq_can_connection')

    def __init__(self):
        self.context = zmq.Context()
    
    def connect(self):
        self.logger.info('Connecting')
        self.tx_socket = self.context.socket(zmq.REQ)
        self.tx_socket.connect('tcp://localhost:5555')
        self.rx_socket = self.context.socket(zmq.SUB)
        self.rx_socket.connect('tcp://localhost:5556')
        self.rx_socket.setsockopt(zmq.SUBSCRIBE, bytes())
        version = self.get_version()
        self._client_name = self.new_client()
        self.logger.info('Connected to zero-m-can version %s, client name %s', version, self._client_name)
    
    def disconnect(self):
        self.logger.info('Disconnecting')
        self.rx_socket.close()
        self.tx_socket.close()

    def get_version(self):
        return self._rpc_call(API_GET_VERSION, [])

    def new_client(self):
        return self._rpc_call(API_NEW_CLIENT, [])

    def _rpc_call(self, func, args):
        bin_request = msgpack.packb((func, args))
        self.tx_socket.send(bin_request)
        bin_response = self.tx_socket.recv()
        response = msgpack.unpackb(bin_response, raw=False)
        result_code, result_value = response
        if result_code != 0:
            raise RuntimeError("RPC call error: {}".format(result_value))
        return result_value

    def send(self, message):
        """ Blocking call, which sends a message onto the bus. """
        self.logger.info('Sending message %s', message)
        self._rpc_call(API_SEND_CAN_MESSAGE, [self._client_name, message.id, message.extended, list(message.data)])
    
    def recv(self):
        """ Blocks until we got a message. """
        self.logger.info('Receiving message')
        while True:
            bindata = self.rx_socket.recv()
            # print(bindata)
            data = msgpack.unpackb(bindata, raw=False)
            client, can_id, can_extended, can_data = data
            if client == self._client_name:
                pass
            else:
                # print(data)
                can_msg = CanMessage(can_id, can_extended, bytes(can_data))
                return can_msg


class ZmqCanServer:
    logger = logging.getLogger('zmq_can_server')

    def __init__(self):
        self.logger.info('starting..')
        self.context = zmq.Context()

        self.incoming = self.context.socket(zmq.REP)
        self.incoming.bind("tcp://*:5555")
        self.publisher = self.context.socket(zmq.PUB)
        self.publisher.bind('tcp://*:5556')

        self.clients = {}
    
    def handle_rpc_call(self):
        handlers = {
            API_GET_VERSION: self.handle_version,
            API_NEW_CLIENT: self.handle_new_client,
            API_SEND_CAN_MESSAGE: self.handle_send_can_message,
        }

        bin_requst = self.incoming.recv()
        request = msgpack.unpackb(bin_requst, raw=False)
        self.logger.debug('Request: %s', request)
        func_code, func_args = request
        func_response = handlers[func_code](*func_args)
        response = [0, func_response]
        self.logger.debug('Response: %s', response)
        bin_response = msgpack.packb(response)
        self.incoming.send(bin_response)

    def handle_version(self):
        return 1

    def handle_new_client(self):
        n = 0
        name = f'client{n}'
        while name in self.clients:
            n += 1
            name = f'client{n}'
        self.clients[name] = name
        return name

    def handle_send_can_message(self, client_name, can_id, can_extended, can_data):
        broadcast = [client_name, can_id, can_extended, can_data]
        self.logger.debug('Distributing message: %s', broadcast)
        bin_broadcast = msgpack.packb(broadcast)
        self.publisher.send(bin_broadcast)


def zmq_can_server():
    """ Start a zeromq can-bus server.
    """
    server = ZmqCanServer()

    while True:
        # Wait for can message:
        server.logger.debug('awaiting message..')
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
    logger = logging.getLogger('zmq_can_dump')
    logger.info('Dumping can messages on zero mq can bus')
    client = ZmqCanConnection()
    client.connect()

    while True:
        msg = client.recv()
        logger.info('Got message %s', msg)

    client.disconnect()


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--server', action='store_true', default=False)
    parser.add_argument('--client', action='store_true', default=False)
    args = parser.parse_args()
    coloredlogs.install(level=logging.DEBUG)

    if args.server:
        zmq_can_server()
    elif args.client:
        zmq_client()
    else:
        zmq_client_dump()
