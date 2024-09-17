#!/usr/bin/env bash

source venv/bin/activate

export TARGET=target-zephyr
export HOOK_FILE=$TARGET/hook.rs
export LOG_FOLDER=$TARGET/runs
export ARCHIVE=$LOG_FOLDER/Hoedur.corpus.tar.zst
export REPORT=$LOG_FOLDER/Hoedur.report.bin.zst

cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
    --config $TARGET/config.yml \
    --debug \
    --trace \
    --hook $HOOK_FILE \
    --hook $TARGET/hook-trace-basic.rs \
    run $1 | tee $TARGET/log-trace.txt | grep -i "debug - \[output\]"