
[![Build Status](https://travis-ci.org/windelbouwman/virtualcan.svg?branch=master)](https://travis-ci.org/windelbouwman/virtualcan)

# Virtual CAN bus

Virtual [CAN bus](https://en.wikipedia.org/wiki/CAN_bus). Send CAN messages over a virtual can bus.

# Use case

A virtual can bus can be useful during development of a CAN based system.

# Usage

## Server

To start the virtual CAN server, use the rust implementation:

    $ cd rust
    $ cargo run --release -- --port 18881

## Python

To use this from python, install the python can package, and next the virtualcan module:

    $ pip install can
    $ cd python
    $ pip install .

Now, from python, use the virtualcan interface as follows:

```python
import can

bus = can.Bus(interface='virtualcan', channel='localhost:18881')
bus.send(can.Message(arbitration_id=13, data=[1, 2, 3, 4, 5]))
```

## C++

Refer to [the C++ demo](cpp/demo).

To build the C++ demo, use cmake:

    $ cd cpp
    $ mkdir build
    $ cd build
    $ cmake ..
    $ make
    $ cd demo
    $ virtualcan_demo.exe

# Roadmap

- C++ implementation of TCP client
- Rust implementation of TCP client/server
- C# implementation of TCP client

# Design

The zero-m-can server listens to port 5555 (req-rep) and port 5556 (pub-sub). Messages are encoded
as msgpack data.

API:
- get_version() -> int
    - get the version of this protocol. Current version: 1
- new_client() -> str
    - Create a new client name. Returns the client name.
- send_message(client: str, id: int, extended: bool, data: list[int]) -> int
    - Send a message on the bus for the given client. Given are an id, extended marker and data.
