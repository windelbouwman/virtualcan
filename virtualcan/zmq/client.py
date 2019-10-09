import logging
import msgpack
import zmq

from . import api
from ..can_message import CanMessage


class ZmqCanConnection:
    """ ZeroMQ can bus client.
    """

    logger = logging.getLogger("zmq_can_connection")

    def __init__(self):
        self.context = zmq.Context()

    def connect(self):
        self.logger.info("Connecting")
        self.tx_socket = self.context.socket(zmq.REQ)
        self.tx_socket.connect("tcp://localhost:5555")
        self.rx_socket = self.context.socket(zmq.SUB)
        self.rx_socket.connect("tcp://localhost:5556")
        self.rx_socket.setsockopt(zmq.SUBSCRIBE, bytes())
        version = self.get_version()
        self._client_name = self.new_client()
        self.logger.info(
            "Connected to zero-m-can version %s, client name %s",
            version,
            self._client_name,
        )

    def disconnect(self):
        self.logger.info("Disconnecting")
        self.rx_socket.close()
        self.tx_socket.close()

    def get_version(self):
        return self._rpc_call(api.API_GET_VERSION, [])

    def new_client(self):
        return self._rpc_call(api.API_NEW_CLIENT, [])

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
        self.logger.info("Sending message %s", message)
        self._rpc_call(
            api.API_SEND_CAN_MESSAGE,
            [self._client_name, message.id, message.extended, list(message.data)],
        )

    def recv(self):
        """ Blocks until we got a message. """
        self.logger.info("Receiving message")
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
