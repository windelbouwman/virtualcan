
FROM ubuntu:24.04

# Install C++ build tools
RUN apt-get update && apt-get install -y --no-install-recommends build-essential cmake ninja-build libgtest-dev

# Install python:
RUN apt-get update && apt-get install -y --no-install-recommends python3 python3-pip python3-pytest

# Install clang-tidy and clang-format
RUN apt-get update && apt-get install -y --no-install-recommends clang-format clang-tidy

# Install ruff:
RUN python3 -m pip install --break-system-packages ruff
