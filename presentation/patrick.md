---
marp: true
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

# Anytime Approximation Strategy

The algorithm has **two main components**:
1. Greedy Top-k Selection
2. Random Edge Selection

---

# Greedy Top-k Selection (Pseudocode)

``` python
    # Only consider unvisited neighbors
    candidates = [(neighbor, weight)
                    for neighbor, weight in graph[current_vertex].items()
                    if neighbor not in visited_vertices]

    if not candidates:
                break

    # pick randomly from top-k heaviest using heapq.nlargest
    top_k_candidates = heapq.nlargest(k, candidates, key=lambda x: x[1])
    neighbor, weight = random.choice(top_k_candidates)

    visited_vertices.add(neighbor)
    current_path.append(neighbor)
    current_weight += weight
    current_vertex = neighbor

```

---

# Random Edge Selection (Pseudocode)

``` python
    # Only consider unvisited neighbors
    candidates = [(neighbor, weight)
                    for neighbor, weight in graph[current_vertex].items()
                    if neighbor not in visited_vertices]

    if not candidates:
        break

    # probability set to 0.3 by default
    if random.random() <= probability:
        neighbor, weight = random.choice(candidates)
    else:
        # pick heaviest neighbor in linear time using max
        neighbor, weight = max(candidates, key=lambda x: x[1])

    visited_vertices.add(neighbor)
    current_path.append(neighbor)
    current_weight += weight
    current_vertex = neighbor

```

---

# Runtime Analysis of the Algorithm

**Let:**

**n** = number of vertices

**E** = number of edges

**Δ** = max degree of a vertex

---

# Top-K Selection Runtime

- **Per vertex iteration**:
  - Scan neighbors: O(Δ)  (Δ = degree of current vertex)
  - Pick top-k using heapq.nlargest: O(Δ)
  - Random choice from top-k: O(1)

- **Total Runtime**:
  - Worst-case O(Δ) = **O(E)**

---

# Random Edge Selection Runtime

- **Per vertex iteration**:
  - Scan neighbors: O(Δ)
  - Random pick: O(1) or max by weight: O(Δ)

- **Total Runtime**:
  - Worst-case O(Δ) = **O(E)**

---

# Anytime Algorithm Total Runtime

- Top-k Selection **O(E)** + Random Edge Selection **O(E)** = **O(E)**
- Algorithm repeats these iterations until time limit is reached,
- **Total Runtime Per Iteration:** O(E)

- **Total Runtime:** unbounded (anytime nature)
  - Can run as long as needed, producing progressively better solutions

---

# Calculating an Upper Bound

Use a **Maximum Spanning Tree**:
- A spanning tree connects all n vertices using exactly n-1 edges
- An MST is the spanning tree with maximum total weight among all trees
- The **Longest Simple Path Weight <= Maximum Spanning Tree Weight**

---

# Patricks Runtime VS Alstons  Runtime

<img src="assets/patricks_runtimes.png" alt="Image 1" width="550" style="display:inline-block;"/>
<img src="assets/alstons_runtime.png" alt="Image 2" width="550" style="display:inline-block;"/>

- Calculated upper bound using **Maximum Spanning Tree**
- Randomness can cause worse results even with more runtime
- Less improvements after 30 seconds

---

![bg contain](../approx_solution_two/assets/patrick_nick.png)

---

![bg contain](../approx_solution_two/assets/patrick_nick_compare.png)
