#!/bin/bash
# Copyright 2022 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../../.." || exit
BUILD_PROFILE=${BUILD_PROFILE:-debug}

# Caveat: has to kill query first.
# `query` tries to remove its liveness record from meta before shutting down.
# If meta is stopped, `query` will receive an error that hangs graceful
# shutdown.
killall bigbytesdb-query || true
sleep 3

killall bigbytesdb-meta || true
sleep 3

for bin in bigbytesdb-query bigbytesdb-meta; do
	if test -n "$(pgrep $bin)"; then
		echo "The $bin is not killed. force killing."
		killall -9 $bin || true
	fi
done

# Wait for killed process to cleanup resources
sleep 1

echo 'Start Meta service HA cluster(3 nodes)...'

mkdir -p ./.bigbytesdb/

nohup ./target/${BUILD_PROFILE}/bigbytesdb-meta -c scripts/ci/deploy/config/bigbytesdb-meta-node-1.toml >./.bigbytesdb/meta-1.out 2>&1 &
python3 scripts/ci/wait_tcp.py --timeout 30 --port 9191

# wait for cluster formation to complete.
sleep 1

nohup ./target/${BUILD_PROFILE}/bigbytesdb-meta -c scripts/ci/deploy/config/bigbytesdb-meta-node-2.toml >./.bigbytesdb/meta-2.out 2>&1 &
python3 scripts/ci/wait_tcp.py --timeout 30 --port 28202

# wait for cluster formation to complete.
sleep 1

nohup ./target/${BUILD_PROFILE}/bigbytesdb-meta -c scripts/ci/deploy/config/bigbytesdb-meta-node-3.toml >./.bigbytesdb/meta-3.out 2>&1 &
python3 scripts/ci/wait_tcp.py --timeout 30 --port 28302

# wait for cluster formation to complete.
sleep 1

echo 'Start bigbytesdb-query node-1'
nohup env RUST_BACKTRACE=1 target/${BUILD_PROFILE}/bigbytesdb-query -c scripts/ci/deploy/config/bigbytesdb-query-node-1.toml --internal-enable-sandbox-tenant >./.bigbytesdb/query-1.out 2>&1 &

echo "Waiting on node-1..."
python3 scripts/ci/wait_tcp.py --timeout 30 --port 9091

echo 'Start bigbytesdb-query node-2'
env "RUST_BACKTRACE=1" nohup target/${BUILD_PROFILE}/bigbytesdb-query -c scripts/ci/deploy/config/bigbytesdb-query-node-2.toml --internal-enable-sandbox-tenant >./.bigbytesdb/query-2.out 2>&1 &

echo "Waiting on node-2..."
python3 scripts/ci/wait_tcp.py --timeout 30 --port 9092

echo 'Start bigbytesdb-query node-3'
env "RUST_BACKTRACE=1" nohup target/${BUILD_PROFILE}/bigbytesdb-query -c scripts/ci/deploy/config/bigbytesdb-query-node-3.toml --internal-enable-sandbox-tenant >./.bigbytesdb/query-3.out 2>&1 &

echo "Waiting on node-3..."
python3 scripts/ci/wait_tcp.py --timeout 30 --port 9093

echo "All done..."
