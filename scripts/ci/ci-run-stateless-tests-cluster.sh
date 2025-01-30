#!/bin/bash
# Copyright 2020-2021 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

echo "Starting Cluster bigbytes-query"

./scripts/ci/deploy/bigbytes-query-cluster-3-nodes.sh

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../../tests" || exit

echo "Starting bigbytes-test"
./bigbytes-test --mode 'cluster' --run-dir 0_stateless
