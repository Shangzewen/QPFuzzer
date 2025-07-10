#!/usr/bin/env bash
# cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
#     --config $TARGET_ZEPHYR/config.yml \
#     --debug \
#     --trace \
#     --hook $HOOK_FILE \
#     --hook $TARGET_ZEPHYR/hook-trace-basic.rs \
#     run $1
source venv/bin/activate
export PATH="/home/asset/.cargo/bin:$PATH" 
# export TARGET=target-zephyr
export TARGET_ZEPHYR=target-zigbee
export HOOK_FILE=$TARGET_ZEPHYR/hook_ref.rs

# cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
#    --config $TARGET_ZEPHYR/config.yml \
#    --debug \
#    --trace \
#    --hook $HOOK_FILE \
#    --hook $TARGET_ZEPHYR/hook-trace-basic.rs \
#    run $1



cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
    --config $TARGET_ZEPHYR/config.yml \
    --debug \
    --trace \
    --hook $HOOK_FILE \
    run $1

