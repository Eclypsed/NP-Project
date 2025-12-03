"""
    name:  Patrick Jabalee-Farrell

    Honor Code and Acknowledgments:

            This work complies with the JMU Honor Code.

           Comments here on your code and submission.
"""
import random
import sys
import time
import heapq

RANDOM_PROB = .3

def approx_solution(graph, time_limit=None, k=3):
    start_time = time.time()

    final_path = []
    total_weight = 0

    # Collect undirected edges once
    edges = []
    for u in graph:
        for v in graph[u]:
            if u < v:
                edges.append((u, v))

    edge_count = len(edges)

    # Loop with greedy random picks
    updated_best = 0
    while True:
        if time_limit is not None and time.time() - start_time >= time_limit:
            return total_weight, final_path
        if updated_best == edge_count:
            break

        u, v = random.choice(edges)
        current_path = [u, v]
        visited_vertices = {u, v}
        current_weight = graph[u][v]
        current_vertex = v

        # Walk greedily
        while True:
            if time_limit is not None and time.time() - start_time >= time_limit:
                return total_weight, final_path

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

        if current_weight > total_weight:
            updated_best = 0
            total_weight = current_weight
            final_path = current_path
        else:
            updated_best += 1

    probability = RANDOM_PROB

    # final loop that is extra random
    while True:
        if time_limit is not None and time.time() - start_time >= time_limit:
            return total_weight, final_path

        u, v = random.choice(edges)
        current_path = [u, v]
        visited_vertices = {u, v}
        current_weight = graph[u][v]
        current_vertex = v

        while True:
            if time_limit is not None and time.time() - start_time >= time_limit:
                return total_weight, final_path

            candidates = [(neighbor, weight)
                          for neighbor, weight in graph[current_vertex].items()
                          if neighbor not in visited_vertices]

            if not candidates:
                break

            if random.random() <= probability:
                neighbor, weight = random.choice(candidates)
            else:
                # pick heaviest neighbor in linear time using max
                neighbor, weight = max(candidates, key=lambda x: x[1])

            visited_vertices.add(neighbor)
            current_path.append(neighbor)
            current_weight += weight
            current_vertex = neighbor

        if current_weight > total_weight:
            total_weight = current_weight
            final_path = current_path


def main():
    DEFAULT_TIME_LIMIT = 2.5

    time_limit = None
    if len(sys.argv) > 1:
        try:
            time_limit = float(sys.argv[1])
        except ValueError:
            print("Time limit must be a number (seconds).")
            sys.exit(1)
    else:
        time_limit = DEFAULT_TIME_LIMIT

    line = sys.stdin.readline().strip().split()
    n, m = int(line[0]), int(line[1])

    graph = {}

    for _ in range(m):
        u, v, w = sys.stdin.readline().strip().split()
        w = float(w)

        if u not in graph:
            graph[u] = {}
        if v not in graph:
            graph[v] = {}

        graph[u][v] = w
        graph[v][u] = w

    total_weight, final_path = approx_solution(graph, time_limit)
    print(f"{total_weight}")
    print(" ".join(final_path))


if __name__ == "__main__":
    main()
