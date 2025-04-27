"""Virtual CAN bus utilities.

There are several options to use this software.

- ZeroMQ: send can messages over ZeroMQ
- TCP/IP: send can messages over a TCP/IP connection

"""

from .can_message import CanMessage


__all__ = ["CanMessage"]
