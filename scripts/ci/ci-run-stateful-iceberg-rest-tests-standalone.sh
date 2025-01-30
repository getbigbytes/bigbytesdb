#!/bin/bash
# Copyright 2020-2021 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

echo "Iceberg Rest integration tests"
echo "Starting standalone BigbytesQuery(debug profile)"
./scripts/ci/deploy/bigbytesdb-query-standalone.sh

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../../tests" || exit

echo "Starting bigbytesdb-test"
./bigbytesdb-test --mode 'standalone' --run-dir 3_stateful_iceberg
