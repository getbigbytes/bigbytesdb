#!/bin/bash
# Copyright 2020-2021 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

echo "Starting standalone BigbytesQuery and BigbytesMeta"
./scripts/ci/deploy/bigbytes-query-standalone.sh

BUILD_PROFILE=${BUILD_PROFILE:-debug}

echo 'Starting bigbytes-sqlsmith tests...'
nohup target/${BUILD_PROFILE}/bigbytes-sqlsmith
