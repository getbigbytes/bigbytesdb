#!/bin/bash
# Copyright 2020-2021 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

echo "*************************************"
echo "* Setting STORAGE_TYPE to S3.       *"
echo "*                                   *"
echo "* Please make sure that S3 backend  *"
echo "* is ready, and configured properly.*"
echo "*************************************"
export STORAGE_TYPE=s3
export STORAGE_S3_BUCKET=testbucket
export STORAGE_S3_ROOT=admin
export STORAGE_S3_ENDPOINT_URL=http://127.0.0.1:9900
export STORAGE_S3_ACCESS_KEY_ID=minioadmin
export STORAGE_S3_SECRET_ACCESS_KEY=minioadmin
export STORAGE_ALLOW_INSECURE=true

echo "Starting standalone BigbytesQuery and BigbytesMeta"
./scripts/ci/deploy/bigbytesdb-query-standalone-native.sh

TEST_HANDLERS=${TEST_HANDLERS:-"mysql,http"}
TEST_PARALLEL=${TEST_PARALLEL:-8}
BUILD_PROFILE=${BUILD_PROFILE:-debug}

RUN_DIR=""
if [ $# -gt 0 ]; then
	RUN_DIR="--run_dir $*"
fi
echo "Run suites using argument: $RUN_DIR"

echo "Starting bigbytesdb-sqllogic tests"
target/${BUILD_PROFILE}/bigbytesdb-sqllogictests --handlers ${TEST_HANDLERS} ${RUN_DIR} --skip_dir management,cluster,explain,tpch,ee --enable_sandbox --parallel ${TEST_PARALLEL}
