#!/bin/bash

set -eu

pushd cpp

# Compile:
cmake -DBUILD_TESTS=ON -G Ninja -S . -B build
cmake --build build

# Test:
./build/test/virtualcan_tests

popd
