"""
    name:  Patrick Jabalee-Farrell

    Honor Code and Acknowledgments:

            This work complies with the JMU Honor Code.

           Comments here on your code and submission.
"""
import random
import sys

# An approximation algorithm for finding the longest path
# Finds the start_k (normally 5) heaviest edges and picks one at random for the
# starting edge. Then greedily finds the 3 heaviest outgoing edges
# and picks one randomly from the 3 until it is finished.
def approx_solution(graph , k=3, start_k=5):
    final_path = []
    total_weight = 0

    # build a list of all edges in the graph for choosing a start
    all_edges = []
    for u in graph:
        for v, w in graph[u].items():
            all_edges.append((u, v, w))

    # sort all edges by weight descending
    all_edges.sort(key=lambda x: x[2], reverse=True)

    # pick randomly from the top start_k edges (normally 5)
    start_edge = random.choice(all_edges[:start_k])
    current = start_edge[0]
    neighbor = start_edge[1]
    weight = start_edge[2]

    visited_vertices = set()
    
    # Add the starting edge to path
    visited_vertices.add(current)
    visited_vertices.add(neighbor)
    final_path.append(current)
    final_path.append(neighbor)
    total_weight += weight

    # Move to the neighbor node
    current = neighbor

    # begin building the longest path
    while True:
        # build an array of all the outgoing edges from a vertex
        candidates = []

        for nbr in graph[current]:
            if nbr not in visited_vertices:
                weight = graph[current][nbr]
                candidates.append((nbr, weight))

        # end the while True loop if the current vertex has no outgoing edges
        if not candidates:
            break

        # sort by weight descending
        candidates.sort(key=lambda x: x[1], reverse=True)

        # select the k heaviest weights (k normally set to 3)
        top_k = candidates[:k]

        # randomly choose one of the selected edges
        neighbor, weight = random.choice(top_k)

        # add the chosen edge to final path, mark as visited, and add weight to total_weight
        visited_vertices.add(neighbor)
        final_path.append(neighbor)

        total_weight += weight

        # move to the next node
        current = neighbor

    return total_weight, final_path


# All modules for CS 412 must include a main method that allows it
# to imported and invoked from other python scripts
def main():
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

    total_weight, final_path = approx_solution(graph)
    print(f"{total_weight}")
    
    print(" ".join(final_path))

if __name__ == "__main__":
    main()