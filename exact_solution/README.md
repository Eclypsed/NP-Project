# Exact Solution

A python solution for calculating the longest path in a weighted graph

## Dependencies

This solution's dependencies can be fond in the
[requirements.txt](./requirements.txt). For easiest setup, use the Nix devenv.
Otherwise you can set up a python virtual environment and install the
dependencies:

**Posix (bash/zsh):**

```shell
python -m venv /path/to/new/virtual/environment
source <venv>/bin/activate
pip install -r requirements.txt
```

### Gradescope

Because gradescope uses an older version of python and does not have does not
have the necessary dependencies, a worse version of the solution (without the
CLI timout or type aliases) can be found in
[cs412_longestpath_exact_gradescope.py](./cs412_longestpath_exact_gradescope.py)

## Usage

All command line options can be viewed using
`python cs412_longestpath_exact.py --help`

To run the program on a specific graph, run
`python cs412_longestpath_exact.py < INPUT_FILE.txt`, optionally specifying a
maximum runtime (in seconds) with `--timeout FLOAT`.

**Example:**

```shell
python cs412_longestpath_exact.py --timeout 60 < ../test_cases/lp_001.txt
```

## Test Cases

The [run_test_cases.sh](./run_test_cases.sh) script will run and time a subset
of test cases from the [test cases folder](../test_cases) that gradually take
longer to finish. Below are my results from running this script (as well as some
additional times from running the rust implementation).

| Test Case                              | Vertices | Edges | Time - Python | Time - Rust (Multithreaded) |
| -------------------------------------- | -------- | ----- | ------------- | --------------------------- |
| [lp_172.txt](../test_cases/lp_172.txt) | 15       | 28    | 0:00.159      | 0:00.004                    |
| [lp_083.txt](../test_cases/lp_083.txt) | 11       | 36    | 0:01.509      | 0:00.027                    |
| [lp_156.txt](../test_cases/lp_156.txt) | 20       | 38    | 0:01.195      | 0:00.020                    |
| [lp_100.txt](../test_cases/lp_100.txt) | 12       | 40    | 0:05.613      | 0:00.080                    |
| [lp_152.txt](../test_cases/lp_152.txt) | 16       | 41    | 0:08.921      | 0:00.119                    |
| [lp_101.txt](../test_cases/lp_101.txt) | 16       | 42    | 0:12.165      | 0:00.177                    |
| [lp_136.txt](../test_cases/lp_136.txt) | 19       | 43    | 0:13.131      | 0:00.174                    |
| [lp_175.txt](../test_cases/lp_175.txt) | 17       | 45    | 0:34.522      | 0:00.429                    |
| [lp_135.txt](../test_cases/lp_135.txt) | 18       | 50    | 1:06.896      | 0:00.720                    |
| [lp_151.txt](../test_cases/lp_151.txt) | 21       | 54    | 3:55.820      | 0:02.478                    |
| [lp_108.txt](../test_cases/lp_108.txt) | 21       | 65    | DNF (>20min)  | 2:02.490                    |

## Importance

The longest path problem is an important problem in graph theory in part due to
its applications in modeling worst-case scenarios and constraints. One such
example is in identifying critical paths in a dependency graph. If a system
forces certain tasks to occur sequentially, the total time required is the
length of the longest chain of dependencies. Another might be analyzing the
worst-case performace of a network where the longest path represents the
greatest latency possible.
