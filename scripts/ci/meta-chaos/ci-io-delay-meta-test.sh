#!/bin/bash
# Copyright 2020-2021 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

BUILD_PROFILE=${BUILD_PROFILE:-debug}

echo "setting up meta chaos.."
./scripts/ci/ci-setup-chaos-meta.sh

HTTP_ADDR="test-bigbytesdb-meta-0.test-bigbytesdb-meta.bigbytesdb.svc.cluster.local:28002,test-bigbytesdb-meta-1.test-bigbytesdb-meta.bigbytesdb.svc.cluster.local:28002,test-bigbytesdb-meta-2.test-bigbytesdb-meta.bigbytesdb.svc.cluster.local:28002"
python3 tests/metaverifier/chaos-meta.py --mode=io/delay/delay=2ms,percent=1 --namespace=bigbytesdb --nodes=${HTTP_ADDR} --total=800 --apply_second=1 --recover_second=10
