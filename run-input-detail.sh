#!/usr/bin/env bash

source venv/bin/activate

# export TARGET=target-zephyr
export TARGET_ZEPHYR=target-zephyr
export TARGET=interval-500-fuzzed-clock-10t

# export HOOK_FILE=$TARGET_ZEPHYR/hook_without_fuzzer.rs
export HOOK_FILE=$TARGET_ZEPHYR/hook.rs
# export HOOK_FILE=$TARGET_ZEPHYR/hook_without_fuzzer220.rs

# cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
#     --config $TARGET/config.yml \
#     --debug \
#     --trace \
#     --hook $HOOK_FILE \
#     run $1

cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
   --config $TARGET/config.yml \
   --debug \
   --trace \
   --hook $HOOK_FILE \
   --hook $TARGET_ZEPHYR/hook-trace-basic.rs \
   run $1
