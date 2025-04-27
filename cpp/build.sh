#!/bin/bash

set -eu

SCRIPT_PATH=$(realpath "$0")
CPP_FOLDER=$(dirname "${SCRIPT_PATH}")
echo "C++ folder: ${CPP_FOLDER}"
pushd ${CPP_FOLDER}

# Compile:
echo "Compile C++ files"
cmake -DBUILD_TESTS=ON -G Ninja -S . -B build
cmake --build build

# Test:
echo "Run unit tests"
./build/test/virtualcan_tests

# Check formatting:
echo "Check C++ style using clang-format"
clang-format **/*.cpp --Werror -n

echo "Done!"

popd
