#!/bin/sh

BUILD_PROFILE="${BUILD_PROFILE:-debug}"

cargo build

killall bigbytes-meta
killall bigbytes-query

rm -rf .bigbytes/meta*

# Generate sample data with a testing load.
make stateless-cluster-test

killall bigbytes-meta
killall bigbytes-query

sleep 2

# Export all meta data from metasrv dir
./target/${BUILD_PROFILE}/bigbytes-metactl export --raft-dir .bigbytes/meta1 >tests/metactl/meta.txt

# Optional: run the test
make metactl-test
