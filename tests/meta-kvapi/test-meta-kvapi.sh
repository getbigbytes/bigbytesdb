#!/bin/sh

set -o errexit

BUILD_PROFILE="${BUILD_PROFILE:-debug}"
BIGBYTES_META="./target/${BUILD_PROFILE}/bigbytes-meta"

echo " === start a single node bigbytes-meta"
chmod +x ${BIGBYTES_META}
${BIGBYTES_META} --single &
METASRV_PID=$!
echo $METASRV_PID

echo " === test kvapi::upsert"
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 1:key1 --value value1
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 1:key2 --value value2
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 1:key3 --value value3
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 2:key1 --value value1
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::upsert --key 2:key2 --value value2

echo " === test kvapi::get"
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::get --key 1:key1
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::get --key 2:key2

echo " === test kvapi::mget"
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::mget --key 1:key1 2:key2

echo " === test kvapi::list"
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::list --prefix 1:
${BIGBYTES_META} --grpc-api-address "127.0.0.1:9191" --cmd kvapi::list --prefix 2:

kill $METASRV_PID
