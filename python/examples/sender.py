
import time
import argparse
import can

parser = argparse.ArgumentParser()
parser.add_argument('--server', default='localhost:18881')
args = parser.parse_args()

bus = can.Bus(interface='virtualcan', channel=args.server)

while True:
    bus.send(can.Message(arbitration_id=13, data=[1,2,3,4,5]))
    time.sleep(0.3)
