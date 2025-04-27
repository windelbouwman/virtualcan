"""This a plugin for the python-can package.

After installation, you can use the virtualcan driver:

    >>> import can
    >>> bus = can.Bus(interface='virtualcan')
    >>> bus.send(can.Message(data=[1, 2, 3, 4, 5]))

"""

import logging
import time
from can import BusABC, Message, BusState
from .can_message import CanMessage
from .tcp.sync_client import SyncTcpClient

logger = logging.getLogger(__name__)


class VirtualCanBus(BusABC):
    """virtualcan interface"""

    def __init__(self, channel=None, **kwargs):
        super().__init__(channel=channel, **kwargs)
        if channel:
            host, port = channel.split(":")
            host = host.strip()
            port = int(port)
            self._client = SyncTcpClient(host=host, port=port)
        else:
            self._client = SyncTcpClient()

    def send(self, msg, timeout=None):
        new_msg = CanMessage(msg.arbitration_id, msg.is_extended_id, msg.data)
        self._client.send_message(new_msg)

    def _recv_internal(self, timeout):
        # print('timeout', timeout)
        msg = self._client.recv_message(timeout=timeout)
        if msg is None:
            can_msg = None
        else:
            can_msg = Message(
                arbitration_id=msg.id,
                is_extended_id=msg.extended,
                data=msg.data,
                timestamp=time.time(),  # Better than nothing...
            )
        return can_msg, False

    @property
    def state(self):
        return BusState.ACTIVE

    @state.setter
    def state(self, new_state):
        pass
