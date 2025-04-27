"""This example connects two can interface to each other.

This can be handy for several use cases:
- Connect hardware can to virtual can
- Bridge two physical can interfaces

This is more of a can module show case than a virtualcan demo.

"""

import argparse
import can
import queue
import threading


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "can0", help="can interface, for example virtualcan:localhost:18881"
    )
    parser.add_argument("can1", help="other can interface, see can0")

    args = parser.parse_args()

    bus0 = create_bus(args.can0)
    bus1 = create_bus(args.can1)

    queue0_to_1 = queue.Queue()
    queue1_to_0 = queue.Queue()

    t1 = threading.Thread(target=recv_func, args=(bus0, queue0_to_1), daemon=True)
    t1.start()

    t2 = threading.Thread(target=recv_func, args=(bus1, queue1_to_0), daemon=True)
    t2.start()

    t3 = threading.Thread(target=send_func, args=(bus0, queue1_to_0), daemon=True)
    t3.start()

    t4 = threading.Thread(target=send_func, args=(bus1, queue0_to_1), daemon=True)
    t4.start()

    print("4 threads are go!")

    input("Press enter to stop this!")


def create_bus(txt):
    interface, channel = txt.split(":", 1)
    bus0 = can.Bus(interface=interface, channel=channel)
    return bus0


def recv_func(bus, q):
    while True:
        msg = bus.recv()
        q.put(msg)


def send_func(bus, q):
    while True:
        msg = q.get()
        bus.send(msg)


if __name__ == "__main__":
    main()
