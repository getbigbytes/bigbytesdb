#!/bin/bash
# Copyright 2020-2021 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

set -ex

echo "start bigbytes-metaverifier with params client:${CLIENT}, number:${NUMBER}, grpc-api-address:${GRPC_ADDRESS}"
echo "START" >/tmp/meta-verifier
# wait for chao-meta.py
sleep 3
/bigbytes-metaverifier --client ${CLIENT} --time 1800 --remove-percent 10 --number ${NUMBER} --grpc-api-address ${GRPC_ADDRESS} || true

while true; do
  sleep 5
done
