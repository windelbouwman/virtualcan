
[![.github/workflows/ci.yaml](https://github.com/windelbouwman/virtualcan/actions/workflows/ci.yaml/badge.svg)](https://github.com/windelbouwman/virtualcan/actions/workflows/ci.yaml)

# Virtual CAN bus

Virtual [CAN bus](https://en.wikipedia.org/wiki/CAN_bus). Send CAN messages over a virtual can bus.

# Use case

A virtual can bus can be useful during development of a CAN based system.

# Usage

## Server

To start the virtual CAN server, use the rust implementation:

    $ cd rust/server
    $ cargo run --release -- --port 18881

## Python

To use this from python, install the python-can package, and next the virtualcan module:

    $ pip install python-can
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
