#!/bin/bash
# Copyright 2020-2021 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

echo "Starting standalone BigbytesQuery and BigbytesMeta"
./scripts/ci/deploy/bigbytes-query-standalone.sh

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../../tests/fuzz" || exit

echo "Starting bigbytes fuzz tests"
python3 fuzz.py
