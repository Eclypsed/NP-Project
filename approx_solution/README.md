# Approximation Solution

A C and Python solution for approximating the longest path in a weighted graph using a randomized greedy algorithm with sampling.

## Dependencies

### C Implementation
- C compiler (gcc, clang, or compatible)
- Standard C libraries
- `graph.c` and `graph.h` helper files
- `hashmap.h` STB-style header only lib

### Python Implementation
Python 3+
This solution has no external dependencies and uses only the Python standard library

### Gradescope

Because gradescope may not have a C compiler available, the Python implementation can be used directly:
[cs412_longestpath_approx.py](./cs412_longestpath_approx.py) (wrapper for C binary) or
[cs412_longestpath_approx_standalone.py](./cs412_longestpath_approx_standalone.py) (pure Python, no compilation needed)

## Usage

### C Implementation

First, build the C binary using the Makefile:

```shell
make
```

Then run the program on a specific graph:

```shell
./bin/cs412_longestpath_approx < INPUT_FILE.txt
```

Optionally specify a maximum runtime (in seconds) with `--time FLOAT`:

**Example:**

```shell
./bin/cs412_longestpath_approx --time 60 < ../test_cases/lp_001.txt
```

### Python Wrapper (Calls C Binary)

The Python wrapper provides an interface to the compiled C binary:

```shell
python cs412_longestpath_approx.py < INPUT_FILE.txt
```

**Note:** This requires building the C binary first with `make`.

### Standalone Python Implementation

All command line options can be viewed using:

```shell
python cs412_longestpath_approx_standalone.py --help
```

To run the program on a specific graph:

```shell
python cs412_longestpath_approx_standalone.py < INPUT_FILE.txt
```

Optionally specify a maximum runtime (in seconds) with `--time FLOAT`:

**Example:**

```shell
python cs412_longestpath_approx_standalone.py --time 60 < ../test_cases/lp_001.txt
```

## Algorithm

This approximation solution uses a randomized greedy approach with three key components:

1. **Greedy Selection with Jumps**: At each step, preferentially choose the highest-weight unvisited edge, but with 15% probability make a random jump to any unvisited neighbor to escape local optima.

2. **Random Walk Extension**: After the greedy phase gets stuck, perform up to 10 additional random steps to explore further.

3. **Sampling**: Repeat the above process with random starting points for the specified time limit and keep the best path found.

### Parameters
- `JUMP_PROB = 0.15` - Probability of making a random jump instead of greedy choice
- `WALK_STEPS = 10` - Number of random walk steps after greedy phase
- `MAX_TIME = 60` (default) - Maximum runtime in seconds

## Test Cases

The [run_test_cases.sh](./run_test_cases.sh) script will run and time all three implementations on a subset of test cases from the [test cases folder](../test_cases). Below are results from running this script with a 1 second time limit per test:

| Test Case | Vertices | Edges | Weight - C | Time - C | Weight - Python Wrapper | Time - Python Wrapper | Weight - Python Standalone | Time - Python Standalone |
|-----------|----------|-------|------------|----------|-------------------------|------------------------|---------------------------|--------------------------|
| lp_172.txt | 15 | 28 | 164 | 0:01.003 | 164 | 0:01.023 | 164 | 0:01.036 |
| lp_083.txt | 11 | 36 | 164 | 0:01.003 | 164 | 0:01.024 | 164 | 0:01.037 |
| lp_156.txt | 20 | 38 | 260 | 0:01.013 | 260 | 0:01.024 | 260 | 0:01.037 |
| lp_100.txt | 12 | 40 | 194 | 0:01.016 | 194 | 0:01.033 | 194 | 0:01.036 |
| lp_152.txt | 16 | 41 | 230 | 0:01.004 | 230 | 0:01.033 | 230 | 0:01.037 |
| lp_101.txt | 16 | 42 | 208 | 0:01.003 | 208 | 0:01.024 | 208 | 0:01.045 |
| lp_136.txt | 19 | 43 | 253 | 0:01.003 | 253 | 0:01.024 | 253 | 0:01.038 |
| lp_175.txt | 17 | 45 | 241 | 0:01.013 | 241 | 0:01.033 | 241 | 0:01.036 |
| lp_135.txt | 18 | 50 | 236 | 0:01.004 | 236 | 0:01.033 | 235 | 0:01.037 |
| lp_151.txt | 21 | 54 | 264 | 0:01.003 | 264 | 0:01.024 | 264 | 0:01.036 |
| lp_108.txt | 21 | 65 | 322 | 0:01.013 | 322 | 0:01.024 | 321 | 0:01.037 |

*Note: Weights may vary between runs due to the randomized nature of the approximation algorithm. With a 0.5 second time limit, all three implementations perform similarly. The C implementation's speed advantage becomes more apparent with longer time limits (e.g., 60 seconds), where it can explore significantly more paths.*

## Importance

The longest path problem is an important problem in graph theory in part due to its applications in modeling worst-case scenarios and constraints. One such example is in identifying critical paths in a dependency graph. If a system forces certain tasks to occur sequentially, the total time required is the length of the longest chain of dependencies. Another might be analyzing the worst-case performance of a network where the longest path represents the greatest latency possible.

Since the longest path problem is NP-hard, approximation algorithms like this one provide practical solutions for larger graphs where exact methods become infeasible.
