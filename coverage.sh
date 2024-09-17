#!/usr/bin/env bash

source venv/bin/activate

export TARGET=target-zephyr
export LOG_FOLDER=$TARGET/runs
export ARCHIVE=$LOG_FOLDER/Hoedur.corpus.tar.zst
export REPORT=$LOG_FOLDER/Hoedur.report.bin.zst
export TRACE_FILE=$LOG_FOLDER/Hoedur.trace.bin.zst

cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
	--debug \
    --trace \
    --trace-file $TRACE_FILE \
    --hook $TARGET/hook-trace.rs \
    --config $TARGET/config.yml \
	run-cov $REPORT $ARCHIVE | grep -i "Coverage" 
