#!/bin/bash
# Copyright 2020-2021 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

export STORAGE_ALLOW_INSECURE=true

echo "Starting standalone BigbytesQuery and BigbytesMeta"
./scripts/ci/deploy/bigbytesdb-query-standalone.sh

TEST_HANDLERS=${TEST_HANDLERS:-"mysql,http"}
TEST_PARALLEL=${TEST_PARALLEL:-8}
BUILD_PROFILE=${BUILD_PROFILE:-debug}

RUN_DIR=""
if [ $# -gt 0 ]; then
	RUN_DIR="--run_dir $*"
fi
echo "Run suites using argument: $RUN_DIR"

echo "Starting bigbytesdb-sqllogic tests"
if [ -z "$RUN_DIR" ]; then
	target/${BUILD_PROFILE}/bigbytesdb-sqllogictests --run_dir temp_table --enable_sandbox --parallel ${TEST_PARALLEL}
fi
target/${BUILD_PROFILE}/bigbytesdb-sqllogictests --handlers ${TEST_HANDLERS} ${RUN_DIR} --skip_dir management,explain_native,ee,temp_table --enable_sandbox --parallel ${TEST_PARALLEL}
