#!/bin/bash
# Copyright 2022 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../../.." || exit
BUILD_PROFILE=${BUILD_PROFILE:-debug}

killall bigbytes-query || true
killall bigbytes-meta || true
sleep 1

for bin in bigbytes-query bigbytes-meta; do
	if test -n "$(pgrep $bin)"; then
		echo "The $bin is not killed. force killing."
		killall -9 $bin || true
	fi
done

# Wait for killed process to cleanup resources
sleep 1

echo 'Start bigbytes-meta...'
nohup target/${BUILD_PROFILE}/bigbytes-meta --single --log-level=INFO &
echo "Waiting on bigbytes-meta 10 seconds..."
python3 scripts/ci/wait_tcp.py --timeout 30 --port 9191

echo 'Start bigbytes-query with native...'
nohup target/${BUILD_PROFILE}/bigbytes-query -c scripts/ci/deploy/config/bigbytes-query-node-native.toml --internal-enable-sandbox-tenant &

echo "Waiting on bigbytes-query 10 seconds..."
python3 scripts/ci/wait_tcp.py --timeout 30 --port 8000
