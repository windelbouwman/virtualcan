""" Can message class.
"""
import json
import binascii


class CanMessage:
    def __init__(self, id, extended, data):
        self.id = id
        self.extended = extended
        self.data = data

    def __repr__(self):
        return f"Can message id={self.id} extended={self.extended} data={self.data}"

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

    def to_bytes(self):
        json_data = self.to_json()
        json_text = json.dumps(json_data)
        bindata = json_text.encode("ascii")
        return bindata

    @classmethod
    def from_bytes(cls, bindata):
        json_text = bindata.decode("ascii")
        json_data = json.loads(json_text)
        return cls.from_json(json_data)
