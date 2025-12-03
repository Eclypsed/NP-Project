#!/usr/bin/env bash

# Exit on errors
set -e

SCRIPT="cs412_longestpath_approx_two.py"

TEST_DIR="../test_cases"

TEST_CASES=(
    "lp_172.txt"
    "lp_083.txt"
    "lp_156.txt"
    "lp_100.txt"
    "lp_152.txt"
    "lp_101.txt"
    "lp_136.txt"
    "lp_175.txt"
    "lp_135.txt"
    "lp_151.txt"
    "lp_108.txt" 
    "lp_badcase.txt"
)

TIME=2.5 # Can be changed

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
