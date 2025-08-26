#!/usr/bin/env bash

source venv/bin/activate
# Add Cargo to PATH (To make sure the cmd_testing script can find cargo correctly since 
# the python subprocess does not know the correct path everytime )
export PATH="~/.cargo/bin:$PATH" 

export TARGET_ZEPHYR=target-zephyr
export TARGET=interval-500-fuzzed-clock-10t

export HOOK_FILE=$TARGET_ZEPHYR/hook.rs
export PREFIX_INPUT=$TARGET_ZEPHYR/inputs/sm_pairing_req.bin



count=0
# while [ $count -lt 100 ]; do
while [ true ]; do
    echo "Run #$count"
    cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
        --config $TARGET/config.yml \
        --debug \
        --trace \
        --hook $HOOK_FILE \
        run \
        --prefix-input $PREFIX_INPUT \
        $1
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
