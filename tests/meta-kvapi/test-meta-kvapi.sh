#!/bin/sh

set -o errexit

BUILD_PROFILE="${BUILD_PROFILE:-debug}"
BIGBYTESDB_META="./target/${BUILD_PROFILE}/bigbytesdb-meta"

echo " === start a single node bigbytesdb-meta"
chmod +x ${BIGBYTESDB_META}
${BIGBYTESDB_META} --single &
METASRV_PID=$!
echo $METASRV_PID

echo " === test kvapi::upsert"
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 1:key1 --value value1
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 1:key2 --value value2
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 1:key3 --value value3
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 2:key1 --value value1
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 2:key2 --value value2

echo " === test kvapi::get"
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::get --key 1:key1
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::get --key 2:key2

echo " === test kvapi::mget"
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::mget --key 1:key1 2:key2

echo " === test kvapi::list"
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::list --prefix 1:
${BIGBYTESDB_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::list --prefix 2:

kill $METASRV_PID
