# Approximation Solution

An anytime Python solution for approximating the longest path in a weighted undirected graph using a randomized greedy algorithm.

## Dependencies

A file for the input which is a weighted undirected graph specified by a line containing the number of vertices n and the number of edges m followed by m lines containing the edges given in u v w format, representing an edge between u and v of weight w. Edges should be listed once.

**Example:**

```shell
3 3
a b 3
b c 4
a c 5
```

## Usage

Without passing an argument for the amount of time to run defaults to 2.5 seconds

```shell
python cs412_longestpath_approx_two.py < INPUT_FILE.txt
```

Passing an argument for amount of time to run

```shell
python cs412_longestpath_approx_two.py 60 < INPUT_FILE.txt
```

## Algorithm

This approximation solution uses a randomized greedy approach with two stages:

1. **Greedy Random Selection**: Starting from a random vertex, greedily find the top k (k defaults to 3) heaviest edges and randomly pick one of them. Continue this until you aren't don't improve your longest path after edges iterations then switch to stage two.

2. **Random Edge Selection**: Once again start from a random vertex, then 30% of the time pick a completely random outgoing edge and 70% of the time pick the heaviest outoging edge. Continue this for the rest of the time remaining.


### Parameters
- `RANDOM_PROB = 0.3` - Percentage chance of choosing a completely random edge.
- `DEFAULT_TIME = 2.5` - Amount of time it runs for if no time argument is passed

## Importance

The longest path problem is an important problem in graph theory in part due to its applications in modeling worst-case scenarios and constraints. One such example is in identifying critical paths in a dependency graph. If a system forces certain tasks to occur sequentially, the total time required is the length of the longest chain of dependencies. Another might be analyzing the worst-case performance of a network where the longest path represents the greatest latency possible.

Since the longest path problem is NP-hard, approximation algorithms like this one provide practical solutions for larger graphs where exact methods become infeasible.
