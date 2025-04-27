#!/bin/bash

set -eu

SCRIPT_PATH=$(realpath "$0")
PYTHON_FOLDER=$(dirname "${SCRIPT_PATH}")
echo "python folder: ${PYTHON_FOLDER}"

pushd ${PYTHON_FOLDER}

export PYTHONPATH=`pwd`
pytest -v .

ruff format --check .
ruff check

popd
