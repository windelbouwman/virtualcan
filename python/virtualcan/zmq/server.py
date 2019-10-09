import zmq
import msgpack
import logging

from . import api


class ZmqCanServer:
    logger = logging.getLogger("zmq_can_server")

    def __init__(self):
        self.logger.info("starting..")
        self.context = zmq.Context()

        self.incoming = self.context.socket(zmq.REP)
        self.incoming.bind("tcp://*:5555")
        self.publisher = self.context.socket(zmq.PUB)
        self.publisher.bind("tcp://*:5556")

        self.clients = {}

    def handle_rpc_call(self):
        handlers = {
            api.API_GET_VERSION: self.handle_version,
            api.API_NEW_CLIENT: self.handle_new_client,
            api.API_SEND_CAN_MESSAGE: self.handle_send_can_message,
        }

        bin_requst = self.incoming.recv()
        request = msgpack.unpackb(bin_requst, raw=False)
        self.logger.debug("Request: %s", request)
        func_code, func_args = request
        func_response = handlers[func_code](*func_args)
        response = [0, func_response]
        self.logger.debug("Response: %s", response)
        bin_response = msgpack.packb(response)
        self.incoming.send(bin_response)

    def handle_version(self):
        return 1

    def handle_new_client(self):
        n = 0
        name = f"client{n}"
        while name in self.clients:
            n += 1
            name = f"client{n}"
        self.clients[name] = name
        return name

    def handle_send_can_message(self, client_name, can_id, can_extended, can_data):
        broadcast = [client_name, can_id, can_extended, can_data]
        self.logger.debug("Distributing message: %s", broadcast)
        bin_broadcast = msgpack.packb(broadcast)
        self.publisher.send(bin_broadcast)
