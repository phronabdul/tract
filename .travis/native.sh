#!/bin/sh

set -ex

rustup update

cargo check --all-targets --workspace --exclude test-tflite --exclude test-metal --exclude tract-metal

./.travis/onnx-tests.sh
./.travis/regular-tests.sh
./.travis/test-harness.sh

if [ -n "$CI" ]
then
    cargo clean
fi

if [ `uname` = "Linux" ]
then
    ./.travis/tflite.sh
fi

if [ `uname` = "Darwin" ]
then
    cargo test -p test-metal
fi


if [ -n "$CI" ]
then
    cargo clean
fi
./.travis/cli-tests.sh
