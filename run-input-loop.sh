#!/usr/bin/env bash

source venv/bin/activate
export PATH="~/.cargo/bin:$PATH" 
# export TARGET=target-zephyr
export TARGET_ZEPHYR=target-zigbee
# export HOOK_FILE=$TARGET_ZEPHYR/hook_ref.rs
export HOOK_FILE=$TARGET_ZEPHYR/hook.rs



count=0
while [ true ]; do
    echo "Run #$count"
    cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
        --config $TARGET_ZEPHYR/config.yml \
        --debug \
        --trace \
        --hook $HOOK_FILE \
        run $1
    ((count++))
    sleep 1
done

#cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
#    --config $TARGET/config.yml \
#    --debug \
#    --trace \
#    --hook $HOOK_FILE \
#    run \
#    $PREFIX_INPUT
