#!/bin/bash

set -eu

pushd rust

cargo build --all
cargo test --all

popd

echo "Done"
