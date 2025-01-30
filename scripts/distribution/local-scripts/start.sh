echo "Stop old Bigbytesdb instances"
killall -9 bigbytesdb-meta
killall -9 bigbytesdb-query
echo "Deploy new Bigbytesdb(standalone)"
ulimit -n 65535
nohup bin/bigbytesdb-meta --config-file=configs/bigbytesdb-meta.toml 2>&1 >meta.log &
sleep 3
# export STORAGE_S3_ENABLE_VIRTUAL_HOST_STYLE=true
nohup bin/bigbytesdb-query --config-file=configs/bigbytesdb-query.toml 2>&1 >query.log &
sleep 3
tail -f meta.log query.log &
