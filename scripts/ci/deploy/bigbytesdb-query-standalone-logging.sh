#!/bin/bash
# Copyright 2022 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../../.." || exit
BUILD_PROFILE=${BUILD_PROFILE:-debug}

killall bigbytesdb-query || true
killall bigbytesdb-meta || true
sleep 1

for bin in bigbytesdb-query bigbytesdb-meta; do
	if test -n "$(pgrep $bin)"; then
		echo "The $bin is not killed. force killing."
		killall -9 $bin || true
	fi
done

# Wait for killed process to cleanup resources
sleep 1

echo 'Start bigbytesdb-meta...'
nohup target/${BUILD_PROFILE}/bigbytesdb-meta --single --log-level=INFO &
echo "Waiting on bigbytesdb-meta 10 seconds..."
python3 scripts/ci/wait_tcp.py --timeout 30 --port 9191

echo 'Start bigbytesdb-query...'
nohup target/${BUILD_PROFILE}/bigbytesdb-query -c scripts/ci/deploy/config/bigbytesdb-query-node-otlp-logs.toml --internal-enable-sandbox-tenant &

echo "Waiting on bigbytesdb-query 10 seconds..."
python3 scripts/ci/wait_tcp.py --timeout 30 --port 8000
