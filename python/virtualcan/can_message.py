"""Can message class."""

import struct
import binascii


class CanMessage:
    def __init__(self, id, extended, data):
        self.id = id
        self.extended = extended
        self.data = data

    def __repr__(self):
        hex_data = " ".join(f"{b:02X}" for b in self.data)
        return f"Can message id={self.id} extended={self.extended} data={hex_data}"

    def __eq__(self, other):
        return (self.id, self.extended, self.data) == (
            other.id,
            other.extended,
            other.data,
        )

    def to_json(self):
        return {
            "id": int(self.id),
            "extended": int(self.extended),
            "bindata": binascii.hexlify(self.data).decode("ascii"),
        }

    @classmethod
    def from_json(cls, json_data):
        id = json_data["id"]
        extended = bool(json_data["extended"])
        data = binascii.unhexlify(json_data["bindata"])
        return cls(id, extended, data)

    _CAN_FMT = ">IBB8s"
    _CAN_FLAGS_EXTENDED = 1

    def to_bytes(self):
        """Serialize this can message to some binary format."""
        flags = 0
        if self.extended:
            flags |= self._CAN_FLAGS_EXTENDED

        data_len = len(self.data)
        bindata = struct.pack(self._CAN_FMT, self.id, flags, data_len, self.data)
        return bindata

    @classmethod
    def from_bytes(cls, bindata):
        """Create a can message from some binary data."""
        can_id, flags, data_len, data = struct.unpack(cls._CAN_FMT, bindata)
        extended = bool(flags & cls._CAN_FLAGS_EXTENDED)
        data = data[:data_len]
        return cls(can_id, extended, data)
