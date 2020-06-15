import socket
import struct
import queue
import threading
from ..can_message import CanMessage


class SyncTcpClient:
    """ Synchronous implementation.
    """

    _FMT = ">I"

    def __init__(self, host="localhost", port=18881):
        self._socket = socket.create_connection((host, port))
        self._rx_queue = queue.Queue()
        self._rx_thread = threading.Thread(target=self._recv_loop, daemon=True)
        self._rx_thread.start()

    def send_message(self, can_message):
        bindata = can_message.to_bytes()
        self._send_packet(bindata)

    def _send_packet(self, data):
        data_len = len(data)
        header_data = struct.pack(self._FMT, data_len)
        self._socket.sendall(header_data + data)

    def recv_message(self, timeout=10.0):
        try:
            msg = self._rx_queue.get(timeout=timeout)
            self._rx_queue.task_done()
        except queue.Empty:
            msg = None
        return msg

    def _recv_loop(self):
        while True:
            bindata = self._recv_packet()
            can_message = CanMessage.from_bytes(bindata)
            self._rx_queue.put(can_message)

    def _recv_packet(self):
        """ Receive a single length prefixed packet. """
        header_data = self._recv_exactly(struct.calcsize(self._FMT))
        data_len, = struct.unpack(self._FMT, header_data)
        data = self._recv_exactly(data_len)
        return data

    def _recv_exactly(self, amount):
        data = bytes()
        while amount > 0:
            new_data = self._socket.recv(amount)
            amount -= len(new_data)
            data += new_data
        return data
