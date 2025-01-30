#!/bin/bash
# Copyright 2022 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../.." || exit

if [ $# -lt 2 ]; then
	echo "usage: run-meta-benchmark.sh client number"
	exit -1
else
	echo "run-meta-benchmark.sh client($1) number($2)"
fi

# Caveat: has to kill query first.
# `query` tries to remove its liveness record from meta before shutting down.
# If meta is stopped, `query` will receive an error that hangs graceful
# shutdown.
killall bigbytes-query
sleep 3

killall bigbytes-meta
sleep 3

for bin in bigbytes-query bigbytes-meta; do
	if test -n "$(pgrep $bin)"; then
		echo "The $bin is not killed. force killing."
		killall -9 $bin
	fi
done

# Wait for killed process to cleanup resources
sleep 1

echo 'Start Meta service HA cluster(3 nodes)...'

nohup ./target/release/bigbytes-meta -c scripts/ci/deploy/config/bigbytes-meta-node-1.toml &
python3 scripts/ci/wait_tcp.py --timeout 30 --port 9191

nohup ./target/release/bigbytes-meta -c scripts/ci/deploy/config/bigbytes-meta-node-2.toml &
python3 scripts/ci/wait_tcp.py --timeout 30 --port 28202

nohup ./target/release/bigbytes-meta -c scripts/ci/deploy/config/bigbytes-meta-node-3.toml &
python3 scripts/ci/wait_tcp.py --timeout 30 --port 28302

echo 'Waiting leader election...'
sleep 5

grpc_address_array=("", "127.0.0.1:9191", "127.0.0.1:28202", "127.0.0.1:28302")
leader_id=$(curl -sL http://0.0.0.0:28101/v1/metrics | grep "^metasrv_server_current_leader_id" | awk '{print $2}')

echo "leader id is $leader_id grpc_address is ${grpc_address_array[$leader_id]}"

if [ $leader_id == "0" ]; then
	echo "leader id is 0, exit"
	exit -1
fi

./target/release/bigbytes-metabench --number $2 --client $1 --grpc-api-address ${grpc_address_array[$leader_id]}
