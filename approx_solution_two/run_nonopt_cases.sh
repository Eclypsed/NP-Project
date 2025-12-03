#!/usr/bin/env bash

# Exit on errors
set -e

SCRIPT="cs412_longestpath_approx_two.py"

TEST_DIR="../test_cases"

TEST_CASES=(
    "lp_badcase.txt"
)

TIME=10 # Can be changed

for case in "${TEST_CASES[@]}"; do
    file="$TEST_DIR/$case"

    if [[ ! -f "$file" ]]; then
        echo "Skipping $file (not a file)"
        continue
    fi

    echo "Running test case $file"
    time python "$SCRIPT" ${TIME} < "$file"
    echo ""
done