
# Virtual CAN bus

Virtual [CAN bus](https://en.wikipedia.org/wiki/CAN_bus). Send CAN messages over a virtual can bus.

# Use case

A virtual can bus can be useful during development of a CAN based system.

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
