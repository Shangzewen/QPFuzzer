#!/usr/bin/env bash

source venv/bin/activate
export PATH="/home/asset/.cargo/bin:$PATH" 
export TARGET=target-zephyr
export HOOK_FILE=$TARGET/hook.rs
export LOG_FOLDER=$TARGET/runs
export ARCHIVE=$LOG_FOLDER/Hoedur.corpus.tar.zst
export REPORT=$LOG_FOLDER/Hoedur.report.bin.zst
export PREFIX_INPUT="$TARGET/inputs/prefix-adv.bin"

cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
    --config $TARGET/config.yml \
    --debug \
    --trace \
    --hook $HOOK_FILE \
    --hook $TARGET/hook-trace-basic.rs \
    run \
    --prefix-input $PREFIX_INPUT $1 | tee $TARGET/log-trace.txt | grep -i "debug - \[output\]"