#!/bin/sh

BUILD_PROFILE="${BUILD_PROFILE:-debug}"

cargo build

killall bigbytesdb-meta
killall bigbytesdb-query

rm -rf .bigbytesdb/meta*

# Generate sample data with a testing load.
make stateless-cluster-test

killall bigbytesdb-meta
killall bigbytesdb-query

sleep 2

# Export all meta data from metasrv dir
./target/${BUILD_PROFILE}/bigbytesdb-metactl export --raft-dir .bigbytesdb/meta1 >tests/metactl/meta.txt

# Optional: run the test
make metactl-test
