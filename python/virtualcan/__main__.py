"""File to allow using:

$ python -m virtualcan

"""

import logging
import argparse
from .cli.tcp import tcp_can_server, tcp_can_client, tcp_client_dump

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--server", action="store_true", default=False)
    parser.add_argument("--client", action="store_true", default=False)
    args = parser.parse_args()
    # coloredlogs.install(level=logging.DEBUG)
    logging.basicConfig(level=logging.DEBUG)

    if args.server:
        tcp_can_server()
    elif args.client:
        tcp_can_client()
    else:
        tcp_client_dump()
