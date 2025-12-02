#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

EXACT="../exact_solution/cs412_longestpath_exact.py"
APPROX="../approx_solution/bin/cs412_longestpath_approx"

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
)

echo "===== Wall-Clock Runtime Measurements ====="
echo

for tc in "${TEST_CASES[@]}"; do
    input_file="$tc"

    echo "Test case: $tc"

    echo "  Exact runtime:"
    time -p python "$EXACT" < "$input_file" > /dev/null

    echo "  Approx runtime:"
    time -p "$APPROX" --time 1 < "$input_file" > /dev/null

    echo
done

