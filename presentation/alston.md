---
marp: true
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

# High-Level Approximation Strategy

The algorithm combines **three major ideas**:
1. Greedy Construction  
2. Randomized Escape  
3. Anytime / Multi-Start Search  

---

# Greedy Construction (Pseudocode)

```
function GREEDY_STEP(G, current, visited):
    candidates = neighbors(current) \ visited
    if candidates is empty:
        return NONE

    score(v) = (degree(v), unvisited_neighbors(v))
    return argmax(score(v)) over candidates
```

---

# Randomized Escape (Pseudocode)

```
function RANDOM_ESCAPE(G, current):
    if random() < JUMP_PROB:
        return random_unvisited_vertex(G)

    neighbors = neighbors(current)
    return random_element(neighbors)
```

---

# Random Walking (Pseudocode)

```
function RANDOM_WALK(G, start, max_steps):
    path = [start]
    current = start

    for i in 1..max_steps:
        neighbors = neighbors(current)
        if neighbors is empty:
            break

        next = random_element(neighbors)
        path.append(next)
        current = next

    return path
```

---

# Runtime Analysis of `sample_path` (Single Iteration)

We analyze the **worst‑case time complexity** of one call to:

```
sample_path(graph, n, start)
```

Let:

- **V = number of vertices**
- **E = number of edges**

---

## Greedy Phase

At each step:

- We scan all neighbors of the current vertex to extract unvisited ones  
- Worst‑case per step: **degree(v)**  
- Across entire run, every vertex is visited at most once → total neighbor scans:

### **O(deg(v)) = O(E)**

---

## Random Walk Extension

The random walk runs for **WALK_STEPS**, which is a constant:
Each of the vertices gathers unvisited neighbors **O(E)**

### **O(1 + E) = O(E)**

---

# Overall Worst-Case Runtime

`T_sample(V, E) = O(E) + O(E) = O(E)`

Since edges dominate, a single sampled greedy+random-walk path runs in **linear time in the number of edges**.

---

![bg contain](../approx_solution/assets/wallclock(python5sec)new.png)

--- 

![bg contain](../approx_solution/assets/wallclock(rust1sec)new.png)

---

![bg contain](../approx_solution/assets/results.png)
