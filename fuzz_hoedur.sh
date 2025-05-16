#!/usr/bin/env bash

source venv/bin/activate

export TARGET_ZEPHYR=target-zephyr
export TARGET=target-zephyr-test

export HOOK_FILE=$TARGET/hook_without_fuzzer.rs
export PREFIX_INPUT=$TARGET/inputs/prefix-adv.bin
export PREFIX_INPUT_2=$TARGET/inputs/feature_req_rsp.bin
export CORPUS_FOLDER=$TARGET/runs


cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
    --config $TARGET/config.yml \
    --hook $HOOK_FILE \
    --trace \
    fuzz \
    --statistics \
    --archive-dir $CORPUS_FOLDER \
    | tee $TARGET/log-fuzzing_test.txt >/dev/null

# cd target-zephyr/runs/
# tar -I zstd -xf Hoedur.corpus.tar.zst
