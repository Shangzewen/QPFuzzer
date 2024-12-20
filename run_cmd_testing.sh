#!/usr/bin/env bash

source venv/bin/activate

export TARGET=target-zephyr
export HOOK_FILE=$TARGET/hook.rs

sudo python3 ./scripts/cmd_testing.py