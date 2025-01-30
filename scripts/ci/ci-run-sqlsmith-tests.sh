#!/bin/bash
# Copyright 2020-2021 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

echo "Starting standalone BigbytesQuery and BigbytesMeta"
./scripts/ci/deploy/bigbytesdb-query-standalone.sh

BUILD_PROFILE=${BUILD_PROFILE:-debug}

echo 'Starting bigbytesdb-sqlsmith tests...'
nohup target/${BUILD_PROFILE}/bigbytesdb-sqlsmith
