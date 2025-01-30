echo "Stop old Bigbytes instances"
killall -9 bigbytes-meta
killall -9 bigbytes-query
echo "Deploy new Bigbytes(standalone)"
ulimit -n 65535
nohup bin/bigbytes-meta --config-file=configs/bigbytes-meta.toml 2>&1 >meta.log &
sleep 3
# export STORAGE_S3_ENABLE_VIRTUAL_HOST_STYLE=true
nohup bin/bigbytes-query --config-file=configs/bigbytes-query.toml 2>&1 >query.log &
sleep 3
tail -f meta.log query.log &
