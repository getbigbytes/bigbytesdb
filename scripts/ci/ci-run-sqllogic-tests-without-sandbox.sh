#!/bin/bash
# Copyright 2020-2021 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

export STORAGE_ALLOW_INSECURE=true

echo "Starting standalone BigbytesQuery and BigbytesMeta"
./scripts/ci/deploy/bigbytes-query-standalone.sh

TEST_HANDLERS=${TEST_HANDLERS:-"mysql,http"}
BUILD_PROFILE=${BUILD_PROFILE:-debug}

RUN_DIR=""
if [ $# -gt 0 ]; then
	RUN_DIR="--run_dir $*"
fi
echo "Run suites using argument: $RUN_DIR"

echo "Starting bigbytes-sqllogic tests"
target/${BUILD_PROFILE}/bigbytes-sqllogictests --handlers ${TEST_HANDLERS} ${RUN_DIR} --skip_dir management,explain_native,ee --parallel 1
