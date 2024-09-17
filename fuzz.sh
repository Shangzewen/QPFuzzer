#!/usr/bin/env bash

source venv/bin/activate

export TARGET=target-zephyr
export HOOK_FILE=$TARGET/hook.rs
# export PREFIX_INPUT=$TARGET/inputs/prefix-adv.bin
export CORPUS_FOLDER=$TARGET/runs

cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
    --config $TARGET/config.yml \
    --hook $HOOK_FILE \
    fuzz \
    --statistics \
    --archive-dir $CORPUS_FOLDER \
    | tee $TARGET/log-fuzzing.txt >/dev/null

# cd target-zephyr/runs/
# tar -I zstd -xf Hoedur.corpus.tar.zst
