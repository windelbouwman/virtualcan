import can

bus = can.Bus(interface="virtualcan")

bus.send(can.Message(arbitration_id=13, data=[1, 2, 3, 4, 5]))
print(bus.recv())
