#!/bin/bash
# Copyright 2020-2021 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

export STORAGE_ALLOW_INSECURE=true

echo "Starting standalone BigbytesQuery and BigbytesMeta"
./scripts/ci/deploy/bigbytesdb-query-management-mode.sh

TEST_HANDLERS=${TEST_HANDLERS:-"mysql,http"}
BUILD_PROFILE=${BUILD_PROFILE:-debug}

RUN_DIR=""
if [ $# -gt 0 ]; then
	RUN_DIR="--run_dir $*"
fi
echo "Run suites using argument: $RUN_DIR"

echo "Starting bigbytesdb-sqllogic tests"
target/${BUILD_PROFILE}/bigbytesdb-sqllogictests --handlers ${TEST_HANDLERS} ${RUN_DIR} --enable_sandbox
