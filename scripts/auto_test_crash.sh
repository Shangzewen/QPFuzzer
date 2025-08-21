#!/bin/bash

counter=0

while [ $counter -lt 209 ]; do
    echo "Running iteration $counter"
    python3 scripts/exithook_auto_testing_input.py ./target-zigbee/meaningful_input/
    counter=$((counter + 1))
done
