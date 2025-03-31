#!/usr/bin/env bash

source venv/bin/activate

export TARGET_ZEPHYR=target-zephyr
export TARGET=interval-500-fuzzed-clock-10t

export HOOK_FILE=$TARGET_ZEPHYR/hook.rs
export PREFIX_INPUT=$TARGET/inputs/prefix-adv.bin
export PREFIX_INPUT_2=$TARGET/inputs/feature_req_rsp.bin
export CORPUS_FOLDER=$TARGET_ZEPHYR/runs


cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
    --config $TARGET_ZEPHYR/config.yml \
    --hook $HOOK_FILE \
    --debug \
    fuzz \
    --statistics \
    --archive-dir $CORPUS_FOLDER \
    | tee $TARGET_ZEPHYR/log-fuzzing.txt

# cd target-zephyr/runs/
# tar -I zstd -xf Hoedur.corpus.tar.zst
