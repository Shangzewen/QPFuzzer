#!/usr/bin/env bash

source venv/bin/activate

export TARGET=target-zephyr
export HOOK_FILE=$TARGET/hook.rs

cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
    --config $TARGET/config.yml \
    --debug \
    --trace \
    --hook $HOOK_FILE \
    run $1
