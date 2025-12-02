---
marp: true
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

# **The Longest Path Problem**

**Computational Complexity Analysis**

---

# Longest Path Problem

- **Input**: Weighted graph G = (V, E, w) where w: E → ℝ⁺
- **Output**: A simple path P with maximum sum of edge weights

**Optimization Version**: Given a weighted graph, find the simple path with
maximum total weight.

**Descision Version (NP)**: Given a weighted graph, does a simple path of weight
≥ k exist?

---

# Example Input and Output

**Input**:

```
8 15 # Vertices, Edges
v2 v1 5
v3 v1 15
v4 v2 10
v5 v2 3
v6 v2 19
v7 v4 14
v8 v2 7
v3 v8 19
v4 v5 15
v2 v7 1
v1 v5 9
v7 v8 17
v3 v6 17
v1 v6 16
v3 v7 20
```

**Output**:

- **Total Weight**: 115
- **Path**: v5 → v4 → v7 → v8 → v3 → v1 → v6 → v2

**Path weights**: 15 + 14 + 17 + 19 + 15 + 16 + 19 = 115 ✓

---

# Certificate Verification

**Certificate**: A sequence of vertices P = [v₁, v₂, ..., vₘ] claimed to be a
longest path

**Verification Algorithm:**

```python
def verify_longest_path(G, certificate, k):
    # Check 1: All vertices in certificate exist in G
    for v in certificate:
        if v not in G.vertices:
            return False
    
    # Check 2: All vertices in certificate are unique (simple path)
    if len(certificate) != len(set(certificate)):
        return False

    total_weight = 0 # Accumulator

    # Check 3: All consecutive pairs form edges in G
    for i in range(len(certificate) - 1):
        edge = G.get_edge(certificate[i], certificate[i+1])

        if edge is None: # Verify edge exists in graph
            return False

        total_weight += edge.weight
    
    # Check 4: Total path weight ≥ k
    return total_weight >= k
```

---

# Verification Complexity

**Runtime Analysis:**

- Check 1: O(|P|) where |P| ≤ |V| - iterate through path
- Check 2: O(|P|) - check each edge and sum weights (O(1) per edge)
- Check 3: O(|P|) - convert to set and compare lengths
- Check 4: O(1) - simple comparison

**Total: O(|V|) - Polynomial in input size!**

**Note**: We can verify that a given path is valid and compute its weight in
polynomial time. This shows the problem is in NP (for the decision version).

---

# NP-Hardness: Reduction from Hamiltonian Path

**Hamiltonian Path Problem** (known NP-Complete):

- Input: Graph G with n vertices
- Output: A path visiting every vertex exactly once (if it exists)

**Reduction Strategy:** Hamiltonian Path ≤ₚ Longest Path (Optimization)

Given instance of Hamiltonian Path (graph G with n vertices):

1. Use the same graph G as input to Longest Path
2. Solve Longest Path to get optimal path P
3. If |P| = n (path visits all vertices), return P as Hamiltonian path
4. Otherwise, no Hamiltonian path exists

**Key Insight:** In any graph, the longest simple path has n vertices ⟺
Hamiltonian path exists

---

# Reduction Example: Hamiltonian Path → Longest Path

**Original Hamiltonian Path Instance:**

```
1 --- 2 --- 3
|           |
4 --- 5 --- 6
```

Question: Find Hamiltonian path (if exists)

**Apply Longest Path Algorithm:**

- Input: Same graph G
- Output: Path [1, 2, 3, 6, 5, 4] with 6 vertices
- Since path uses all 6 vertices → this IS a Hamiltonian path ✓

**If no Hamiltonian path existed:**

- Longest path would have < 6 vertices
- We'd return "No Hamiltonian path"

---

# Reduction Correctness

**Forward (⇒):** If G has Hamiltonian path H of n vertices:

- H visits all n vertices exactly once
- Any longest path must have ≤ n vertices (can't exceed total)
- Therefore longest path has exactly n vertices
- Longest Path algorithm will find a path of length n ✓

**Backward (⇐):** If Longest Path returns path P with n vertices:

- P is simple (no repeated vertices)
- P has n vertices in a graph with n total vertices
- Therefore P visits every vertex exactly once
- P is a Hamiltonian path ✓

**Polynomial Time:** Reduction takes O(1) - just run algorithm and check path
length Since Hamiltonian Path is NP-Complete, Longest Path is NP-Hard!

---

# Implementation: Backtracking Solution

```python
def longest_path(
    n: int, graph: dict[int, dict[int, float]]
) -> tuple[list[int], float]:
    
    def recurse(
        start: int, weight: float, visited: list[bool]
    ) -> tuple[list[int], float]:
        visited[start] = True
        if start not in graph:
            return ([start], weight)
        
        best_path: list[int] = [start]
        best_weight: float = weight
        
        for adj in graph[start].keys():
            if visited[adj]:
                continue
            # DOMINANT OPERATION: Recursive exploration
            res = recurse(adj, weight + graph[start][adj], visited)
            visited[adj] = False
            
            new_path, new_weight = res
            if new_weight > best_weight:
                best_weight = new_weight
                best_path = [start, *new_path]
        
        return (best_path, best_weight)
    
    # Try starting from each vertex
    return reduce(
        lambda x, y: x if x[1] > y[1] else y,
        map(lambda start: recurse(start, 0, [False] * n), range(n)),
    )
```

---

# Analytical Runtime Analysis

**Dominant Operation:** Recursive `recurse()` call exploring all possible paths

**Worst Case Analysis:**

- Start from each of n vertices: n starting points
- At each vertex, try extending to each unvisited neighbor
- At depth d, we have d visited vertices, (n - d) unvisited
- Total simple paths explored: n × (n-1) × (n-2) × ... × 1 = n!

**Key Operations:**

- `recurse()` recursive calls: **O(n!)**
- Inside each call: O(degree) to iterate neighbors
- Worst case (complete graph): degree = O(n)
- Total: **O(n! × n) = O(n!)**

**Space Complexity:** O(n) for recursion stack and visited array

This factorial runtime is the dominant term and confirms the exponential nature
of the problem!

---

# Empirical Runtime Analysis

**Test Setup:** (Test cases timed with zsh time built-in)

| Test Case  | Vertices | Edges | Time - Python | Time - Rust |
| ---------- | -------- | ----- | ------------- | ----------- |
| lp_172.txt | 15       | 28    | 0:00.159      | 0:00.004    |
| lp_083.txt | 11       | 36    | 0:01.509      | 0:00.027    |
| lp_156.txt | 20       | 38    | 0:01.195      | 0:00.020    |
| lp_100.txt | 12       | 40    | 0:05.613      | 0:00.080    |
| lp_152.txt | 16       | 41    | 0:08.921      | 0:00.119    |
| lp_101.txt | 16       | 42    | 0:12.165      | 0:00.177    |
| lp_136.txt | 19       | 43    | 0:13.131      | 0:00.174    |
| lp_175.txt | 17       | 45    | 0:34.522      | 0:00.429    |
| lp_135.txt | 18       | 50    | 1:06.896      | 0:00.720    |
| lp_151.txt | 21       | 54    | 3:55.820      | 0:02.478    |
| lp_108.txt | 21       | 65    | DNF (>20min)  | 2:02.490    |

---

# Runtime Growth Visualization

```
Runtime (seconds)
    150 |                                          *
        |
    100 |
        |
     50 |
        |
     10 |                                    *
      1 |                          *
        |                    *
    0.1 |              *
        |        *  *
   0.01 |  *  *
        |_______________________________________________
          4   5   6   7   8   9   10    Input Size (n)

Growth Pattern: Factorial (n!)
Each increment multiplies runtime by approximately n
```

---

# Complexity Summary

**Problem Class:** NP-Complete (Optimization version is NP-Hard)

**Why NP-Hard?**

- Reduction from Hamiltonian Path in O(1) time ✓
- Reduction is polynomial and correctness proven ✓

**Practical Implications:**

- No known polynomial-time algorithm
- Backtracking solution: O(n! × |E|)
- Infeasible for large graphs (n > 20)
- Approximation algorithms needed for practical use

---

# Approximation 1

Alston's Stuff

---

# Approximation 2

Patrick's Stuff
