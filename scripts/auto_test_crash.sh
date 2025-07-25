#!/bin/bash

counter=0

while [ $counter -lt 11 ]; do
    echo "Running iteration $counter"
    python3 scripts/exithook_auto_testing_input.py ./target-zephyr/meaningful_input_multi_version/v350/
    counter=$((counter + 1))
done