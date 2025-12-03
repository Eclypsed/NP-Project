---
marp: true
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

# High-Level Approximation Strategy

The algorithm combines **two major ideas**:
1. Greedy 
2. Random Exploration 

---

# Greedy Construction

``` python
def GREEDY_STEP(G, current, visited):
    # All neighbors of current that are not yet visited
    candidates = [v for v in G[current] if v not in visited]
    if not candidates:
        return None

    # Score(v): degree + number of unvisited neighbors
    def SCORE(v):
        degree = len(G[v])
        unseen = sum(1 for u in G[v] if u not in visited)
        return (degree, unseen)

    # Return the candidate with maximum score
    return max(candidates, key=SCORE)

```

---

# Random Exploration

``` python
def RANDOM_EXPLORE(G, current, visited, JUMP_PROB):
    r = random.random()      # number in [0, 1)

    # With probability JUMP_PROB, explore randomly
    if r < JUMP_PROB:
        candidates = [v for v in G[current] if v not in visited]
        if not candidates:
            return None
        return random.choice(candidates)

    # Otherwise, follow greedy rule
    return GREEDY_STEP(G, current, visited)
```

---

# Runtime Analysis of `sample_path`

We analyze the worst-case runtime of one greedy + random-jump path.

The graph is stored with adjacency lists,
so scanning neighbors depends on the number of edges.

---

## Greedy Phase (with Random Jumps)

At each step:

- Scan all neighbors of the current vertex → **O(degree(current))**
- Filter unvisited neighbors
- Pick best-weight or random-choice → **O(1)**

Each vertex becomes `current` at most once,
so each edge is scanned at most **twice**.

---

## Overall Worst-Case Runtime

Total neighbor scanning:

degree(v₁) + degree(v₂) + … + degree(vₙ)
= **2E = O(E)**

Thus:

**T_sample(E) = O(E)**

Each sampled path runs in  
**linear time in the number of edges.**

---
![bg right:100% 100%](../approx_solution/assets/wallclock(python5sec)new.png)
![bg left:100% 100%](../approx_solution/assets/wallclock(rust1sec)new.png)

---

![bg contain](../approx_solution/assets/results.png)

---

![bg contain](../approx_solution/assets/graph_time_0_001s_finalcolors.png)

---

![bg contain](../approx_solution/assets/graph_time_0_01s_finalcolors.png)

---

![bg contain](../approx_solution/assets/graph_time_0_1s_finalcolors.png)

---

![bg contain](../approx_solution/assets/graph_time_1s_finalcolors.png)

---

![bg contain](../approx_solution/assets/graph_time_5s_finalcolors.png)

