import time
import random
from collections import defaultdict
import click

type Graph = dict[int, dict[int, float]]

JUMP_PROB = 0.15


def sample_path(graph: Graph, n: int, start: int) -> tuple[list[int], float]:
    """Build a randomized greedy path with jumps and random walk extension."""
    visited = [False] * n
    path = [start]
    visited[start] = True
    total = 0.0
    current = start
    
    # Greedy loop with random jumps
    while current in graph:
        unvisited = [(to, w) for to, w in graph[current].items() if not visited[to]]
        if not unvisited:
            break
        
        max_weight = max(w for _, w in unvisited)
        best = [(to, w) for to, w in unvisited if w == max_weight]
        
        # Random jump or greedy choice
        chosen = random.choice(unvisited if len(unvisited) > 1 and random.random() < JUMP_PROB else best)
        
        current, weight = chosen
        visited[current] = True
        path.append(current)
        total += weight
    
    return (path, total)


def find_best_path(graph: Graph, n: int, max_time: float) -> tuple[list[int], float]:
    """Sample random paths within time limit and return the best."""
    start_time = time.time()
    best = max(
        (sample_path(graph, n, random.randint(0, n - 1)) 
         for _ in iter(lambda: time.time() - start_time < max_time, False)),
        key=lambda x: x[1],
        default=([], -1.0)
    )
    return best


@click.command()
@click.option("--time", "-t", type=float, default=60.0, help="Maximum time to run (seconds)")
def main(time: float):
    first_line = input().split()
    n, m = int(first_line[0]), int(first_line[1])
    
    graph: Graph = defaultdict(dict)
    vertices: list[str] = [""] * n
    vertex_map: dict[str, int] = {}
    
    for _ in range(m):
        u, v, w = input().split()
        for name in (u, v):
            if name not in vertex_map:
                vertex_map[name] = len(vertex_map)
                vertices[vertex_map[name]] = name
        
        weight = float(w)
        ui, vi = vertex_map[u], vertex_map[v]
        graph[ui][vi] = graph[vi][ui] = weight
    
    path, weight = find_best_path(graph, n, time)
    print(f"{weight:.0f}")
    print(" ".join(vertices[node] for node in path))


if __name__ == "__main__":
    main()
