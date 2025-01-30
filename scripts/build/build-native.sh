#!/bin/bash
# Copyright 2020-2021 The Bigbytes Authors.
# SPDX-License-Identifier: Apache-2.0.

set -e

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/../.." || exit

echo "Build(NATIVE) start..."
RUSTFLAGS="-C target-cpu=native" cargo build --bin=bigbytes-query --bin=bigbytes-meta --bin=bigbytes-metactl --release
echo "All done..."
