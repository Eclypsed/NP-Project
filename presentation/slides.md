---
marp: true
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
style: |
  :root {
    font-size: 1.75rem;  /* scale down everything */
  }
---

# **The Longest Path Problem**

**Computational Complexity Analysis**

---

# Longest Path Problem

- **Input**: Weighted graph $G = (V, E, w)\ \text{where}\ w: E \to \mathbb{R}^{+}$
- **Output**: A simple path P with maximum sum of edge weights

**Optimization Version (NP-Hard)**: Given a weighted graph, find the simple path with maximum total weight.

**Descision Version (NP-Complete)**: Given a weighted graph, does a simple path of weight $\geq k$ exist?

---

# Optimization Example

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

# Shortest vs Longest Path

Why can't we use a shortest path algorithm like Bellman-Ford (Polynomial) to solve Longest Path?

**Attempt #1 - Negate every weight**:
Definitionally, Shortest Path is _not_ required to be simple, Longest Path _is_.

- Bellman-Ford will stop if it encounters a negative weight cycle to prevent an infinite loop.
- Even in the presence of a positive weight cycle, verticies in Longest Path can not be repeated and so it will never loop.

**Note**: Allowing repeated veritices and positive weight cycle detection in Longest Path would make it polynomial. Conversely, requiring Shortest Path to be simple would make it NP-Hard.

---

# Shortest vs Longest Path

Why can't we use a shortest path algorithm like Bellman-Ford (Polynomial) to solve Longest Path?

**Attempt #2 - Use reciprocal weights**:
This doesn't work either because the reciprocal of a sum is not equal to the sum of reciprocals (e.g. $\frac{1}{2 + 3} \neq \frac{1}{2} + \frac{1}{3}$)

**Counter-Example**

- Path A: $w_1 = 1, w_2 = 4$
- Path B: $w_3 = 2, w_2 = 2$

Longest Path: $(1 + 4) > (2 + 2) \therefore\ \text{Longest Path:}\ (w_1, w_2)$
Shortest Path + Reciprocals: $(1 + \frac{1}{4}) > (\frac{1}{2} + \frac{1}{2}) \therefore\ \text{Longest Path:}\ (w_3, w_4)$

---

# NP (Certificate Verification)

**Certificate**: A sequence of vertices P = [v₁, v₂, ..., vₘ] claimed to be a longest path

```python
def verify_longest_path(G, certificate, k):
    # Check 1: O(|P|) - All vertices in certificate exist in G
    for v in certificate:
        if v not in G.vertices:
            return False

    # Check 2: O(|P|) - All vertices in certificate are unique (simple path)
    if len(certificate) != len(set(certificate)):
        return False

    # Check 3: O(|P|) - All consecutive pairs form edges in G
    total_weight = 0 # Accumulator
    for i in range(len(certificate) - 1):
        edge = G.get_edge(certificate[i], certificate[i+1])
        if edge is None: # Verify edge exists in graph
            return False

        total_weight += edge.weight

    # Check 4: O(1) - Total path weight ≥ k
    return total_weight >= k
```

---

# NP-Hardness

**Hamiltonian Path Problem** (known NP-Complete):

- Input: Unweighted Graph $G = (V, E)$
- Output: A path visiting every vertex exactly once (if it exists)

**Reduction Strategy:** Hamiltonian Path $\leq_p$ Longest Path

If the Hamiltonian Path Poblem can be reduced to Longest Path in polynomial time, then the Longest Path problem is also NP-Hard.

---

# Reduction

**Reduce Hamiltonian Path → Longest Path in polynomial time:**

Assign every edge in the graph a weight of 1.
$G = (V, E) \to G = (V, E, w)\ \text{where}\ w: E \to 1$. $O(|E|)$

**Reframe decision:**

Does a path of total weight $\geq |V| - 1$ exist?

- If YES, then the path found by Longest Path is a Hamiltonian path.
- If NO, then no Hamiltonian Path exists.

---

# Reduction Correctness

**Forward (⇒):** If $G$ has Hamiltonian path $P$:

- $P$ visits all vertices exactly once
- $P$ uses exactly $|V| - 1$ edges.
- All edges have weight 1 so the total weight of $P$ is $|V| - 1$
- Thus there exists a simple path of total weight $\geq |V| - 1$, so Longest Path answers YES ✓

---

# Reduction Correctness

**Backward (⇐):** If Longest Path returns path $P$ of weight $\geq |V| - 1$:

- $P$ is simple (no repeated vertices)
- $P$ uses exactly $|V|$ vertices in a graph with $|V|$ total vertices
- Therefore $P$ visits every vertex exactly once
- $P$ is a Hamiltonian path ✓

**Conclusion:** Reduction runs in polynomial time, therefore, because Hamiltonian Path is NP-Complete, Longest Path is NP-Hard

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

This factorial runtime is the dominant term and confirms the exponential nature of the problem!

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

![bg 90%](./assets/exact-solution-graph.png)

---

# Approximation 1

Alston's Stuff

---

# Approximation 2

Patrick's Stuff
