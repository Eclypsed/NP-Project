#!/usr/bin/env bash

# Exit on errors
set -e

SCRIPT="cs412_longestpath_approx.py"

TEST_DIR="../test_cases"

TEST_CASES=(
    "lp_badcase.txt"
)

TIME=0.5 # Can be changed

for case in "${TEST_CASES[@]}"; do
    file="$TEST_DIR/$case"

    if [[ ! -f "$file" ]]; then
        echo "Skipping $file (not a file)"
        continue
    fi

    echo "Running test case $file"
    time python "$SCRIPT" --time ${TIME} < "$file"
    echo ""
done
