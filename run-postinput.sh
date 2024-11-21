#!/usr/bin/env bash

source venv/bin/activate
# Add Cargo to PATH (To make sure the cmd_testing script can find cargo correctly since 
# the python subprocess does not know the correct path everytime )
export PATH="/home/asset/.cargo/bin:$PATH" 

export TARGET=target-zephyr
export HOOK_FILE=$TARGET/hook.rs
export PREFIX_INPUT="$TARGET/inputs/prefix-adv.bin"
export PREFIX_INPUT2="$TARGET/inputs/feature_rep_rsp.bin"



cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
    --config $TARGET/config.yml \
    --debug \
    --trace \
    --hook $HOOK_FILE \
    run \
    --prefix-input $PREFIX_INPUT \
    $1

#cargo run --release --bin hoedur-arm 3>&1 1>&2 2>&3 -- \
#    --config $TARGET/config.yml \
#    --debug \
#    --trace \
#    --hook $HOOK_FILE \
#    run \
#    $PREFIX_INPUT
