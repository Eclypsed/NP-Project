#!/usr/bin/env bash

# Exit on errors
set -e

SCRIPT="cs412_longestpath_approx_two.py"

TEST_DIR="../test_cases"

TEST_CASES=(
    "lp_151.txt"
    "lp_184.txt"
    "lp_208.txt"
    "lp_251.txt"
    "lp_324.txt"
)

TIME=60 # Can be changed

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
