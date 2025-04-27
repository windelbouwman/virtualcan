#!/bin/bash

set -eu

pushd python

export PYTHONPATH=`pwd`
pytest -v .

ruff format --check .
ruff check

popd
