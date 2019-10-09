import unittest
from virtualcan import CanMessage


class CanMessageTestCase(unittest.TestCase):
    def test_equality(self):
        can_msg = CanMessage(1337, False, bytes([1, 2, 3, 4]))
        can_msg2 = CanMessage(1337, False, bytes([1, 2, 3, 4]))
        self.assertEqual(can_msg, can_msg2)

    def test_json_encoding(self):
        can_msg = CanMessage(1337, False, bytes([1, 2, 3, 4]))
        json_data = can_msg.to_json()
        can_msg2 = CanMessage.from_json(json_data)
        self.assertEqual(can_msg, can_msg2)

    def test_binary_encoding(self):
        can_msg = CanMessage(1337, False, bytes([1, 2, 3, 4]))
        json_data = can_msg.to_bytes()
        can_msg2 = CanMessage.from_bytes(json_data)
        self.assertEqual(can_msg, can_msg2)
