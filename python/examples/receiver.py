"""Connect to CAN bus and print all messages."""

import argparse
import can

parser = argparse.ArgumentParser()
parser.add_argument("--server", default="localhost:18881")
args = parser.parse_args()

bus = can.Bus(interface="virtualcan", channel=args.server)

counter = 0
while True:
    print(bus.recv())
