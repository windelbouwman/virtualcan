""" File to allow using:

$ python -m virtualcan

"""

import logging
import argparse


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--server", action="store_true", default=False)
    parser.add_argument("--client", action="store_true", default=False)
    args = parser.parse_args()
    # coloredlogs.install(level=logging.DEBUG)
    logging.basicConfig(level=logging.DEBUG)

    # Choose between TCP/IP and ZeroMQ:
    use_zmq = False

    if use_zmq:
        from .cli.zmq import zmq_can_server, zmq_client, zmq_client_dump

        if args.server:
            zmq_can_server()
        elif args.client:
            zmq_client()
        else:
            zmq_client_dump()
    else:
        from .cli.tcp import tcp_can_server, tcp_can_client, tcp_client_dump

        if args.server:
            tcp_can_server()
        elif args.client:
            tcp_can_client()
        else:
            tcp_client_dump()
