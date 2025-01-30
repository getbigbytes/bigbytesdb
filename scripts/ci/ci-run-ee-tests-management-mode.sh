#!/bin/bash
# Copyright 2020-2021 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

export STORAGE_ALLOW_INSECURE=true

echo "calling test suite"
echo "Starting standalone BigbytesQuery(debug)"
./scripts/ci/deploy/bigbytes-query-management-mode.sh

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../../tests" || exit

echo "Starting bigbytes-test"
./bigbytes-test $1 --mode 'standalone' --run-dir management
